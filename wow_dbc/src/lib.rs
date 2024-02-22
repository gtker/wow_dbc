//! Implementation of the [DBC (`DataBaseClient`) file format for World of Warcraft 1.12, 2.4.3.8606, and 3.3.5.12340](https://wowdev.wiki/DBC).
//! This is auto generated from `.xml` files [in the github repository](https://github.com/gtker/wow_dbc/tree/main/rxml/xml).
//!
//! DBC files are inside [the MPQ files](https://wowdev.wiki/MPQ) that are included with the client.
//! This library does not deal with MPQ files.
//! You will have to extract the DBC files from the MPQ using another tool.
//!
//! # Usage
//!
//! The [`vanilla_tables`], [`tbc_tables`], and [`wrath_tables`] modules contain submodules with table definitions.
//! Each table always has two types (using [`vanilla_tables::item_class`] as an example):
//!
//! 1. The table: [`ItemClass`](`vanilla_tables::item_class::ItemClass`). Which implements [`DbcTable`].
//! 2. The table row: [`ItemClassRow`](`vanilla_tables::item_class::ItemClassRow`). Which is accessed through [`DbcTable::rows`].
//!
//! Tables that have a primary key additionally have a type suffixed with `Key` ([`ItemClassKey`](`vanilla_tables::item_class::ItemClassKey`)),
//! and the table ([`ItemClass`](`vanilla_tables::item_class::ItemClass`)) implements the [`Indexable`] trait.
//!
//! [`Gender`] and [`SizeClass`] are in the crate root because they are used in multiple tables and have been de-duplicated in order to reduce the amount of types in the crate.
//!
//! # Installation
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! wow_dbc = { version = "0.3.0", features = ["vanilla", "tbc", "wrath"] }
//! ```
//!
//! Or add it with [cargo edit](https://github.com/killercup/cargo-edit):
//! ```bash
//! cargo add wow_dbc --features "vanilla tbc wrath"
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
//! # Tests
//!
//! Published builds do not have any tests for individual tables,
//! but these can be built by changing `BUILD_TESTS` in `rxml/src/main.rs` of the repository.
//! These require the original DBC files from the client which can not be republished.
//! The DBC files must be placed in the root of the repository.
//!

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
// This requires fields knowing about the sizes of enums
#![allow(clippy::useless_conversion)]
#![allow(non_camel_case_types)]
#![warn(
clippy::perf,
clippy::correctness,
clippy::style,
clippy::missing_const_for_fn,
clippy::missing_errors_doc,
clippy::missing_panics_doc,
clippy::doc_markdown,
clippy::unseparated_literal_suffix,
missing_docs
)]

use std::io::{Read, Write};

pub(crate) mod error;

pub use error::*;

#[allow(unused)]
pub(crate) mod header;

#[allow(missing_docs, clippy::unnecessary_cast)]
#[cfg(feature = "vanilla")]
pub mod vanilla_tables;

#[allow(missing_docs, clippy::unnecessary_cast)]
#[cfg(feature = "tbc")]
pub mod tbc_tables;

#[allow(missing_docs, clippy::unnecessary_cast)]
#[cfg(feature = "wrath")]
pub mod wrath_tables;

mod tys;

#[allow(unused)]
pub use tys::*;

#[allow(unused)]
mod util;

/// Main trait for the crate. Implemented by all tables in [`vanilla_tables`].
pub trait DbcTable: Sized {
    /// Will be the name of the implementing type suffixed with `Row`.
    type Row;

    /// The name of the DBC file _with_ `.dbc` at the end.
    const FILENAME: &'static str;

    /// Array of all rows. Are not guaranteed to be in any order.
    fn rows(&self) -> &[Self::Row];
    /// Mutable array of all rows. Are not guaranteed to be in any order.
    fn rows_mut(&mut self) -> &mut [Self::Row];

    /// Read table from bytes.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Read::read_exact`].
    ///
    /// Will error with [`InvalidHeaderError`] if the magic numbers (`0x43424457`) at the start of the file do not match.
    fn read(b: &mut impl Read) -> Result<Self, DbcError>;
    /// Write to bytes.
    ///
    /// The string block will always start with a zero byte so that a string index of 0 is always an empty string.
    ///
    /// This is not guaranteed to create the exact same binary as is shipped with the game, but it will be semantically the same.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Write::write_all`].
    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error>;
}

/// Implemented by tables that have a primary key.
///
/// This is a separate trait instead of just implementing [`std::ops::Index`] and [`std::ops::IndexMut`] since
/// those traits do not return [`Option`]s and only have the possibility of panicking on invalid keys.
///
/// The original DBCs do not really respect primary/foreign keys, so this just seemed like it would make everything more annoying.
pub trait Indexable: DbcTable {
    /// Key used to index into the table. Same name as the table suffixed with `Key`.
    type PrimaryKey;

    /// Gets the primary key, if present. Internally this is just [`std::iter::Iterator::find`] since the
    /// items are not guaranteed to be ordered nor even be present.
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row>;

    /// Gets the primary key, if present. Internally this is just [`std::iter::Iterator::find`] since the
    /// items are not guaranteed to be ordered nor even be present.
    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row>;
}
