//! This crate contains code to convert the [DBC (`DataBaseClient`) file format for World of Warcraft 1.12, 2.4.3.8606, and 3.3.5.12340](https://wowdev.wiki/DBC) into a sqlite database.
//! The crate also provides functionality to write the generated sqlite tables to dbc files.
//!
//! # Usage
//! wow_dbc_converter.exe <expansion version> -i <input_path_of_dbcs> -o <output_path_of_sqlite_database>
//!
//!
//! # Installation
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! wow_dbc_converter = { version = "0.2.0", features = ["vanilla", "tbc", "wrath"] }
//! ```
//!
//! Or add it with [cargo edit](https://github.com/killercup/cargo-edit):
//! ```bash
//! cargo add wow_dbc_converter --features "vanilla tbc wrath"
//! ```
//!
//! # Features
//!
//! By default no features are enabled.
//! The following features are valid:
//! * `vanilla`, for 1.12 client data.
//! * `tbc`, for 2.4.3.8606 client data.
//! * `wrath`, for 3.3.5.12340 client data.
//!
//! To add only a specific version, remove the unneeded ones from the `features` list above.
//!
//! # Library
//! The library contains the sqlite bindings for vanilla, tbc, and wrath.
//!
//! <expansion>::generate_dbc_for(&name, &conn, &mut writer) will generate a dbc file for the given table name
//!   from the provided sqlite connection to the provided writer.
//!
//! <expansion>::<dbc_name>_from_rows creates a dbc of the given type from sqlite rows.
//!
//! <expansion>::<TableName> provides the sql creation statement, the insert sql, and the select sql for the table.
//!

use std::fmt::{Display, Formatter};
use crate::error::SqliteError;

pub mod error;

#[allow(missing_docs, clippy::unnecessary_cast)]
#[cfg(feature = "tbc")]
pub mod tbc_tables_sqlite;
#[allow(missing_docs, clippy::unnecessary_cast)]
#[cfg(feature = "vanilla")]
pub mod vanilla_tables_sqlite;
#[allow(missing_docs, clippy::unnecessary_cast)]
#[cfg(feature = "wrath")]
pub mod wrath_tables_sqlite;

#[derive(Debug, Copy, Clone, clap::ValueEnum)]
pub enum Expansion {
    Vanilla,
    BurningCrusade,
    Wrath,
}

impl Display for Expansion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Expansion::Vanilla => "Vanilla (1.12.x.y)",
                Expansion::BurningCrusade => "The Burning Crusade (2.4.3.8606)",
                Expansion::Wrath => "Wrath of the Lich King (3.3.5.12340)",
            }
        )
    }
}


pub fn get_table_sql(name: &str, expansion: Expansion) -> Result<(&'static str, &'static str, &'static str), SqliteError> {
    match expansion {
        #[cfg(feature = "vanilla")]
        Expansion::Vanilla => vanilla_tables_sqlite::get_sql_for(name),
        #[cfg(feature = "tbc")]
        Expansion::BurningCrusade => tbc_tables_sqlite::get_sql_for(name),
        #[cfg(feature = "wrath")]
        Expansion::Wrath => wrath_tables_sqlite::get_sql_for(name),
        _ => Err(SqliteError::EnumError(format!("{expansion} is not a valid expansion. Are the right features enabled?")))

    }
}

pub fn generate_dbc_for(expansion: Expansion, name: &str, conn: &rusqlite::Connection, writer: &mut dyn std::io::Write) -> Result<(), SqliteError> {
    match expansion {
        #[cfg(feature = "vanilla")]
        Expansion::Vanilla => vanilla_tables_sqlite::generate_dbc_for(&name, &conn, writer),
        #[cfg(feature = "tbc")]
        Expansion::BurningCrusade => tbc_tables_sqlite::generate_dbc_for(&name, &conn, writer),
        #[cfg(feature = "wrath")]
        Expansion::Wrath => wrath_tables_sqlite::generate_dbc_for(&name, &conn, writer),
        _ => Err(SqliteError::EnumError(format!("{expansion} is not a valid expansion. Are the right features enabled?")))
    }
}