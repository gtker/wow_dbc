mod error;
mod tbc_tables_sqlite;
mod vanilla_tables_sqlite;
mod wrath_tables_sqlite;

pub(crate) use error::*;

use std::fs::read_dir;
use std::path::PathBuf;

fn workspace_dir() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assert!(p.pop());
    p
}

fn vanilla_dir() -> PathBuf {
    workspace_dir().join("vanilla-dbc")
}

fn tbc_dir() -> PathBuf {
    workspace_dir().join("tbc-dbc")
}

fn wrath_dir() -> PathBuf {
    workspace_dir().join("wrath-dbc")
}

#[derive(Debug, Clone)]
struct Options {
    sqlite_path: PathBuf,
}

fn main() {
    let vanilla = Options {
        sqlite_path: PathBuf::from("/tmp/output/data.sqlite"),
    };

    for file in read_dir(vanilla_dir()).unwrap().filter_map(|a| a.ok()) {
        if !file.file_type().unwrap().is_file() {
            continue;
        }

        let file_name = file.file_name();
        let file_name = file_name.to_string_lossy();

        let file_contents = std::fs::read(file.path()).unwrap();

        vanilla_tables_sqlite::write_to_sqlite(
            file_name.as_ref(),
            &mut file_contents.as_slice(),
            &vanilla,
        )
        .unwrap();
    }

    let tbc = Options {
        sqlite_path: PathBuf::from("/tmp/output/tbc.sqlite"),
    };

    for file in read_dir(tbc_dir()).unwrap().filter_map(|a| a.ok()) {
        if !file.file_type().unwrap().is_file() {
            continue;
        }

        let file_name = file.file_name();
        let file_name = file_name.to_string_lossy();

        let file_contents = std::fs::read(file.path()).unwrap();

        tbc_tables_sqlite::write_to_sqlite(file_name.as_ref(), &mut file_contents.as_slice(), &tbc)
            .unwrap();
    }

    let wrath = Options {
        sqlite_path: PathBuf::from("/tmp/output/wrath.sqlite"),
    };

    for file in read_dir(wrath_dir()).unwrap().filter_map(|a| a.ok()) {
        if !file.file_type().unwrap().is_file() {
            continue;
        }

        let file_name = file.file_name();
        let file_name = file_name.to_string_lossy();

        let file_contents = std::fs::read(file.path()).unwrap();

        wrath_tables_sqlite::write_to_sqlite(
            file_name.as_ref(),
            &mut file_contents.as_slice(),
            &wrath,
        )
        .unwrap();
    }
}
