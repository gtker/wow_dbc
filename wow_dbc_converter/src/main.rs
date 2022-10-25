mod error;
mod tbc_tables_sqlite;
mod vanilla_tables_sqlite;
mod wrath_tables_sqlite;

pub(crate) use error::*;

use std::fs::{create_dir, read, read_dir};
use std::path::PathBuf;
use std::process::exit;

use clap::Parser;

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
                println!(
                    "Unable to open directory for reading: '{}' with error '{}'",
                    options.input_path.display(),
                    e
                );
                exit(1);
            }
        };
        input_directory
            .filter_map(|a| {
                if let Some(a) = a.ok() {
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
        println!(
            "Input directory '{}' is not either a directory or a file.",
            options.input_path.display()
        );
        println!("Exiting.");
        exit(1);
    };

    for file in files {
        let contents = match read(&file) {
            Ok(e) => e,
            Err(e) => {
                println!("Failed to read file '{}': '{}'.", file.display(), e);
                if options.strict_mode {
                    println!("Exiting.");
                    exit(1);
                } else {
                    println!("Skipping file and continuing.");
                    continue;
                }
            }
        };

        if contents.len() == 0 {
            println!(
                "'{}' is an empty file. This can happend for some Vanilla files.",
                file.display()
            );
            println!("Skipping file and continuing.");
            continue;
        }

        let file_name = match file.file_name() {
            Some(e) => e,
            None => {
                println!("Failed to get file name for '{}'", file.display());
                if options.strict_mode {
                    println!("Exiting.");
                    exit(1);
                } else {
                    println!("Skipping file and continuing.");
                    continue;
                }
            }
        };
        let file_name = file_name.to_string_lossy();
        let file_name = file_name.as_ref();

        match options.expansion {
            Expansion::Vanilla => {
                match vanilla_tables_sqlite::write_to_sqlite(
                    file_name,
                    &mut contents.as_slice(),
                    &options,
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        println!(
                            "Error occurred during processing of '{}': '{}'",
                            file.display(),
                            e
                        );
                        if options.strict_mode {
                            println!("Exiting.");
                            exit(1);
                        } else {
                            println!("Skipping file and continuing.");
                            continue;
                        }
                    }
                }
            }
            Expansion::BurningCrusade => {
                match tbc_tables_sqlite::write_to_sqlite(
                    file_name,
                    &mut contents.as_slice(),
                    &options,
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        println!(
                            "Error occurred during processing of '{}': '{}'",
                            file.display(),
                            e
                        );
                        if options.strict_mode {
                            println!("Exiting.");
                            exit(1);
                        } else {
                            println!("Skipping file and continuing.");
                            continue;
                        }
                    }
                }
            }
            Expansion::Wrath => {
                match wrath_tables_sqlite::write_to_sqlite(
                    file_name,
                    &mut contents.as_slice(),
                    &options,
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        println!(
                            "Error occurred during processing of '{}': '{}'",
                            file.display(),
                            e
                        );
                        if options.strict_mode {
                            println!("Exiting.");
                            exit(1);
                        } else {
                            println!("Skipping file and continuing.");
                            continue;
                        }
                    }
                }
            }
        }
    }
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
