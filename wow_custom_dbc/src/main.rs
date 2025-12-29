

use wow_dbc_converter::{tbc_tables_sqlite,vanilla_tables_sqlite,wrath_tables_sqlite, Expansion};

use std::fs::create_dir_all;
use std::path::PathBuf;
use clap::Parser;
use rusqlite::Connection;
use std::fs::{read_dir, read_to_string};
use std::process::exit;



#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to put all the generated dbc files in
    ///
    /// Creates a dbc directory in the current working directory by default
    #[arg(short = 'o', long)]
    output_path: Option<PathBuf>,

    /// Input directory of the DML files
    ///
    /// If this is a single sql file only that file will be converted.
    /// If this is an sqlite database, each table in the sqlite database will be converted into dbc files
    /// If it is a directory all '.sql' files in the directory will be inserted into the sqlite database and converted to dbc files
    #[arg(short = 'i', long)]
    input_path: PathBuf,

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
    sqlite_path: PathBuf,
    expansion: Expansion,
    strict_mode: bool,
}


fn main() {
    let args = Args::parse();

    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let output_path = args.output_path.unwrap_or_else(|| cwd.join("dbc_output"));

    // We'll use a temporary file or in-memory for the processing if not specified,
    // but here we'll just create a 'custom.sqlite' in the output directory
    if !output_path.exists() {
        let _ = create_dir_all(&output_path);
    }
    let sqlite_path = output_path.join("custom_patch.sqlite");

    let options = Options {
        input_path: args.input_path,
        output_path,
        sqlite_path,
        expansion: args.dbc_version,
        strict_mode: args.strict_mode,
    };

    // 1. Initialize Database
    let mut conn = Connection::open(&options.sqlite_path).unwrap_or_else(|e| {
        fatal_error(format!("Failed to open SQLite database: {}", e));
    });

    // 2. Read SQL files
    let mut sql_files = Vec::new();
    if options.input_path.is_dir() {
        let dir = read_dir(&options.input_path).unwrap_or_else(|e| {
            fatal_error(format!("Failed to read input directory: {}", e));
        });
        for entry in dir.filter_map(|e| e.ok()) {
            if entry.path().extension().and_then(|s| s.to_str()) == Some("sql") {
                sql_files.push(entry.path());
            }
        }
    } else if options.input_path.extension().and_then(|s| s.to_str()) == Some("sql") {
        sql_files.push(options.input_path.clone());
    }

    if sql_files.is_empty() {
        println!("No SQL files found to process.");
        return;
    }

    // 3. Process each SQL file
    for file in sql_files {
        let table_name = file.file_stem().unwrap().to_string_lossy();
        let sql_content = read_to_string(&file).unwrap_or_else(|e| {
            fatal_error(format!("Failed to read SQL file {:?}: {}", file, e));
        });

        println!("Processing table: {}", table_name);

        // Start a transaction for this specific table
        let tx = conn.transaction().unwrap_or_else(|e| {
            fatal_error(format!("Failed to start transaction: {}", e));
        });

        // 1. Get the DDL (CREATE TABLE statement) from the library
        match wow_dbc_converter::get_table_sql(&table_name, options.expansion) {
            Ok(sql) => {
                if let Err(e) = tx.execute_batch(sql.0) {
                    recoverable_error(
                        format!("Failed to execute DDL for {}: {}", table_name, e),
                        "Skipping table",
                        &options,
                    );
                    let _ = tx.rollback();
                    continue;
                }
            }
            Err(e) => {
                fatal_error(format!("Failed to get SQL schema for {}: {}", table_name, e));
            }
        };

        // 2. Execute the SQL content (Custom INSERTS) inside the transaction
        if let Err(e) = tx.execute_batch(&sql_content) {
            recoverable_error(
                format!("Failed to execute SQL inserts for {}: {}", table_name, e),
                "Skipping table",
                &options,
            );
            let _ = tx.rollback();
            continue;
        }

        // Commit the transaction for this table
        if let Err(e) = tx.commit() {
            fatal_error(format!("Failed to commit transaction for {}: {}", table_name, e));
        }
    }

    // 4. Generate DBC files
    export_database_to_dbcs(&options);
}

fn recoverable_error(error: impl AsRef<str>, continue_text: impl AsRef<str>, options: &Options) {
    println!("Error: {}", error.as_ref());
    if options.strict_mode {
        println!("Strict mode enabled. Exiting.");
        exit(1);
    } else {
        println!("{}. Continuing...", continue_text.as_ref());
    }
}

fn fatal_error(error: impl AsRef<str>) -> ! {
    eprintln!("Fatal Error: {}", error.as_ref());
    exit(1);
}


fn export_database_to_dbcs(options: &Options) {
    let conn = match Connection::open(&options.sqlite_path) {
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
                        let result = wow_dbc_converter::generate_dbc_for(options.expansion, &name, &conn, &mut writer);

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


