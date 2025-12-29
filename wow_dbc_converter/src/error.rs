use std::error::Error;
use std::fmt::{Display, Formatter};
use wow_dbc::DbcError;

#[derive(Debug)]
pub enum SqliteError {
    Rusqlite(rusqlite::Error),
    EnumError(String),
    DbcError(DbcError),
    FilenameNotFound { name: String },
}

impl Display for SqliteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SqliteError::Rusqlite(v) => v.fmt(f),
            SqliteError::DbcError(v) => v.fmt(f),
            SqliteError::EnumError(v) => write!(f, "EnumError: {}", v),
            SqliteError::FilenameNotFound { name } => {
                write!(f, "Unknown DBC file encountered: '{}'", name)
            }
        }
    }
}

impl Error for SqliteError {}

impl From<rusqlite::Error> for SqliteError {
    fn from(v: rusqlite::Error) -> Self {
        Self::Rusqlite(v)
    }
}

impl From<DbcError> for SqliteError {
    fn from(v: DbcError) -> Self {
        Self::DbcError(v)
    }
}

