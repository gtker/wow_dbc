mod error;
mod tbc_tables_sqlite;
mod vanilla_tables_sqlite;
mod wrath_tables_sqlite;

pub(crate) use error::*;

use std::fs::{create_dir, read, read_dir};
use std::path::PathBuf;
use std::process::exit;

use clap::Parser;
use rusqlite::Connection;

/// Convert DBC files to SQLite databases
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output path of the SQLite file.
    ///
    /// If this is a directory a file named 'dbcs.sqlite' will be created in that directory.
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

#[derive(Debug, Copy, Clone, clap::ValueEnum)]
pub(crate) enum Expansion {
    Vanilla,
    BurningCrusade,
    Wrath,
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
}

fn apply_file(options: &Options, file: &PathBuf) {
    let contents = match read(&file) {
        Ok(e) => e,
        Err(e) => {
            recoverable_error(
                format!("Failed to read file '{}': '{}'.", file.display(), e),
                "Skipping file and continuing",
                &options,
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
            &options,
        );
        return;
    }

    let file_name = match file.file_name() {
        Some(e) => e,
        None => {
            recoverable_error(
                format!("Failed to get file name for '{}'", file.display()),
                "Skipping file and continuing",
                &options,
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
                        &options,
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
                        &options,
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
                        &options,
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
        cwd
    } else {
        println!("Current working directory is invalid.");
        println!("Exiting.");
        exit(1);
    };

    let input_path = if let Some(p) = args.input_path {
        p
    } else {
        cwd.clone()
    };

    let output_path = if let Some(p) = args.output_path {
        if !p.exists() && p.extension().is_none() {
            println!("Output directory '{}' does not exist.", p.display());
            if args.strict_mode {
                println!("Exiting.");
                exit(1);
            } else {
                println!(
                    "Attempting to create directory '{}' and continuing.",
                    p.display()
                );
                match create_dir(&p) {
                    Ok(_) => {
                        println!("Successfully created directory '{}'", p.display());
                        println!("Continuing.");
                    }
                    Err(_) => {
                        println!("Failed to create directory '{}'", p.display());
                        println!("Exiting.");
                        exit(1);
                    }
                }
            }
        }

        if p.is_dir() {
            p.join("dbc.sqlite")
        } else {
            p
        }
    } else {
        cwd.join("dbcs.sqlite")
    };

    Options {
        input_path,
        output_path,
        expansion: args.dbc_version,
        strict_mode: args.strict_mode,
    }
}
