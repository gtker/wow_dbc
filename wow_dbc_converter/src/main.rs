mod error;
mod tbc_tables_sqlite;
mod vanilla_tables_sqlite;
mod wrath_tables_sqlite;

pub(crate) use error::*;
use std::fmt::{Display, Formatter};

use std::fs::{create_dir_all, read, read_dir};
use std::path::PathBuf;
use std::process::exit;

use clap::Parser;
use rusqlite::Connection;

use wow_dbc_converter::Expansion;

/// Convert DBC files to SQLite databases
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output path of the SQLite file.
    ///
    /// If this is a directory a file named '{dbc_version}_dbcs.sqlite' will be created in that directory.
    /// Otherwise the full filename will be used.
    ///
    /// Defaults to the current working directory.
    #[arg(short = 'o', long)]
    output_path: Option<PathBuf>,

    /// Input path of the DBC files.
    ///
    /// If this is a single file only that file will be converted.
    /// If it is a directory all '.dbc' files in the directory will be converted to the same SQLite file.
    ///
    /// Defaults to the current working directory.
    #[arg(short = 'i', long)]
    input_path: Option<PathBuf>,

    /// Which expansion version to read the DBC files as.
    ///
    /// Must be correct otherwise errors will likely occur.
    #[arg(value_enum)]
    dbc_version: Expansion,

    /// Exits on all errors instead of continuing.
    #[arg(short = 's', long)]
    strict_mode: bool,
}



#[derive(Debug, Clone)]
struct Options {
    input_path: PathBuf,
    output_path: PathBuf,
    expansion: Expansion,
    strict_mode: bool,
}

fn main() {
    let args = Args::parse();
    let options = options(args);

    let files = if options.input_path.is_dir() {
        let input_directory = match read_dir(&options.input_path) {
            Ok(e) => e,
            Err(e) => {
                fatal_error(format!(
                    "Unable to open directory for reading: '{}' with error '{}'",
                    options.input_path.display(),
                    e
                ));
            }
        };
        input_directory
            .filter_map(|a| {
                if let Ok(a) = a {
                    if let Some(extension) = a.path().extension() {
                        if extension.to_string_lossy().as_ref() == "dbc" {
                            Some(a.path())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    } else if options.input_path.is_file() {
        vec![options.input_path.clone()]
    } else {
        fatal_error(format!(
            "Input directory '{}' is not either a directory or a file.",
            options.input_path.display()
        ));
    };

    files.iter().for_each(|a| apply_file(&options, a));

    post_process_database(&options);
}

fn apply_file(options: &Options, file: &PathBuf) {
    let contents = match read(file) {
        Ok(e) => e,
        Err(e) => {
            recoverable_error(
                format!("Failed to read file '{}': '{}'.", file.display(), e),
                "Skipping file and continuing",
                options,
            );
            return;
        }
    };

    if contents.is_empty() {
        recoverable_error(
            format!(
                "'{}' is an empty file. This can happend for some Vanilla files.",
                file.display()
            ),
            "Skipping file and continuing",
            options,
        );
        return;
    }

    let file_name = match file.file_name() {
        Some(e) => e,
        None => {
            recoverable_error(
                format!("Failed to get file name for '{}'", file.display()),
                "Skipping file and continuing",
                options,
            );
            return;
        }
    };
    let file_name = file_name.to_string_lossy();
    let file_name = file_name.as_ref();

    let mut conn = match Connection::open(&options.output_path) {
        Ok(e) => e,
        Err(e) => {
            fatal_error(format!(
                "Unable to open SQLite database '{}' because of error: '{}'",
                options.output_path.display(),
                e
            ));
        }
    };
    match options.expansion {
        Expansion::Vanilla => {
            match vanilla_tables_sqlite::write_to_sqlite(
                &mut conn,
                file_name,
                &mut contents.as_slice(),
            ) {
                Ok(_) => {}
                Err(e) => {
                    recoverable_error(
                        format!(
                            "Error occurred during processing of '{}': '{}'",
                            file.display(),
                            e
                        ),
                        "Skipping file and continuing",
                        options,
                    );
                }
            }

        }
        Expansion::BurningCrusade => {
            match tbc_tables_sqlite::write_to_sqlite(&mut conn, file_name, &mut contents.as_slice())
            {
                Ok(_) => {}
                Err(e) => {
                    recoverable_error(
                        format!(
                            "Error occurred during processing of '{}': '{}'",
                            file.display(),
                            e
                        ),
                        "Skipping file and continuing",
                        options,
                    );
                }
            }
        }
        Expansion::Wrath => {
            match wrath_tables_sqlite::write_to_sqlite(
                &mut conn,
                file_name,
                &mut contents.as_slice(),
            ) {
                Ok(_) => {}
                Err(e) => {
                    recoverable_error(
                        format!(
                            "Error occurred during processing of '{}': '{}'",
                            file.display(),
                            e
                        ),
                        "Skipping file and continuing",
                        options,
                    );
                }
            }
        }
    }
}

fn recoverable_error(error: impl AsRef<str>, continue_text: impl AsRef<str>, options: &Options) {
    println!("{}", error.as_ref());
    if options.strict_mode {
        println!("Exiting.");
        exit(1);
    } else {
        println!("{}", continue_text.as_ref());
    }
}

fn fatal_error(error: impl AsRef<str>) -> ! {
    println!("{}", error.as_ref());
    println!("Exiting.");
    exit(1);
}

fn options(args: Args) -> Options {
    let cwd = if let Ok(cwd) = std::env::current_dir() {
        cwd.canonicalize().unwrap_or(cwd)
    } else {
        println!("Current working directory is invalid.");
        println!("Exiting.");
        exit(1);
    };

    let input_path = if let Some(p) = args.input_path {
        p.canonicalize().unwrap_or(p)
    } else {
        cwd.clone()
    };

    let default_file_name = match args.dbc_version {
        Expansion::Vanilla => "vanilla_dbcs.sqlite",
        Expansion::BurningCrusade => "burning_crusade_dbcs.sqlite",
        Expansion::Wrath => "wrath_dbcs.sqlite",
    };

    let output_path = if let Some(p) = args.output_path {
        if p.exists() {
            println!("Output path '{}' exists.", p.display());
            if !p.is_file() {
                println!("Output path '{}' is not a file.", p.display());
                p.join(default_file_name)
            } else {
                println!("Output path '{}' is a file.", p.display());
                p
            }
        } else {
            println!("Output path '{}' does not exist.", p.display());

            if let Some(parent) = p.parent() {
                if parent.exists() {
                    println!("Parent '{}' exists.", parent.display());
                    println!("Creating new file at '{}'", p.display());

                    p
                } else {
                    if args.strict_mode {
                        println!("Exiting.");
                        exit(1);
                    }

                    println!("Creating path '{}' and all parents.", p.display());

                    match create_dir_all(parent) {
                        Ok(_) => {
                            println!("Successfully created directory '{}'", parent.display());
                        }
                        Err(e) => {
                            println!("Failed to create directory '{}': '{}'", parent.display(), e);
                            println!("Exiting.");
                            exit(1);
                        }
                    }

                    p
                }
            } else {
                println!("Parent for '{}' does not exist.", p.display());

                p.join(default_file_name)
            }
        }
    } else {
        cwd.join(default_file_name)
    };
    let output_path = output_path.canonicalize().unwrap_or(output_path);

    println!();
    println!("Input path: '{}'", input_path.display());
    println!("Output path: '{}'", output_path.display());
    println!(
        "Strict mode: {}",
        if args.strict_mode {
            "Enabled"
        } else {
            "Disabled"
        }
    );
    println!("DBC Version: {}", args.dbc_version);

    Options {
        input_path,
        output_path,
        expansion: args.dbc_version,
        strict_mode: args.strict_mode,
    }
}

fn post_process_database(options: &Options) {
    let conn = match Connection::open(&options.output_path) {
        Ok(e) => e,
        Err(e) => {
            println!("Unable to open SQLite database for post-processing: {}", e);
            return;
        }
    };

    let mut stmt = match conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name") {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to prepare table list statement: {}", e);
            return;
        }
    };

    let names = stmt.query_map([], |row| row.get::<_, String>(0))
        .and_then(|mapped| mapped.collect::<Result<Vec<_>, _>>());

    match names {
        Ok(n) => {
            println!("\nTables present in database ({} total):", n.len());

            // Create the 'dbc' directory at the target location
            let mut dbc_dir = options.output_path.clone();
            if dbc_dir.is_file() {
                dbc_dir.pop();
            }
            dbc_dir.push("dbc");

            if let Err(e) = create_dir_all(&dbc_dir) {
                println!("Failed to create directory '{}': {}", dbc_dir.display(), e);
                return;
            }

            for name in n {
                println!("  - {}", name);

                let mut file_path = dbc_dir.clone();
                file_path.push(format!("{}.dbc", name));

                match std::fs::File::create(&file_path) {
                    Ok(file) => {
                        let mut writer = std::io::BufWriter::new(file);
                        let result = match options.expansion {
                            Expansion::Vanilla => {
                                vanilla_tables_sqlite::generate_dbc_for(&name, &conn, &mut writer)
                            }
                            Expansion::BurningCrusade => {
                                tbc_tables_sqlite::generate_dbc_for(&name, &conn, &mut writer)
                            }
                            Expansion::Wrath => {
                                wrath_tables_sqlite::generate_dbc_for(&name, &conn, &mut writer)
                            }
                        };

                        if let Err(e) = result {
                            recoverable_error(
                                format!("Failed to generate DBC data for '{}': {}", name, e),
                                "Continuing to next table",
                                options,
                            );
                        }
                    }
                    Err(e) => {
                        println!("    ! Failed to create file '{}': {}", file_path.display(), e);
                    }
                }
            }
            println!("\nDBC files generated in: {}", dbc_dir.display());
            println!("\nEmpty DBC files created in: {}", dbc_dir.display());
        }
        Err(e) => println!("Error retrieving table names: {}", e),
    }
}
