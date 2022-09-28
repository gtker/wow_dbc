use std::error::Error;
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

/// Main error enum. Returned from [`crate::DbcTable::read`].
#[derive(Debug)]
pub enum DbcError {
    /// IO errors.
    Io(std::io::Error),
    /// Errors from invalid enum values.
    InvalidEnum(InvalidEnumError),
    /// Errors from converting bytes to strings.
    String(FromUtf8Error),
    /// Errors related to headers.
    InvalidHeader(InvalidHeaderError),
}

impl Display for DbcError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DbcError::Io(i) => i.fmt(f),
            DbcError::InvalidEnum(i) => i.fmt(f),
            DbcError::String(i) => i.fmt(f),
            DbcError::InvalidHeader(i) => i.fmt(f),
        }
    }
}

impl Error for DbcError {}

impl From<std::io::Error> for DbcError {
    fn from(i: std::io::Error) -> Self {
        Self::Io(i)
    }
}

impl From<FromUtf8Error> for DbcError {
    fn from(e: FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<InvalidHeaderError> for DbcError {
    fn from(e: InvalidHeaderError) -> Self {
        Self::InvalidHeader(e)
    }
}

/// Errors from reading the header of the DBC file.
#[derive(Debug)]
pub enum InvalidHeaderError {
    /// The magic value was not `0x43424457`, but was instead [`InvalidHeaderError::MagicValue::actual`].
    MagicValue {
        /// Value gotten instead of magic header.
        actual: u32,
    },
    /// The reported `record_size` did not match the precomputed.
    RecordSize {
        /// Expected value.
        expected: u32,
        /// Actual value read.
        actual: u32,
    },
    /// The reported amount of fields did not match the precomputed.
    FieldCount {
        /// Expected value.
        expected: u32,
        /// Actual value read.
        actual: u32,
    },
}

impl Display for InvalidHeaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidHeaderError::RecordSize { expected, actual } => {
                write!(
                    f,
                    "invalid record size. Expected '{}', got '{}'",
                    expected, actual
                )
            }
            InvalidHeaderError::FieldCount { expected, actual } => write!(
                f,
                "invalid field count. Expected '{}', got '{}'",
                expected, actual
            ),
            InvalidHeaderError::MagicValue { actual } => {
                write!(f, "invalid header magic: '{}'", actual)
            }
        }
    }
}

impl Error for InvalidHeaderError {}

/// Error for values outside of allowed enumerators.
#[derive(Debug)]
pub struct InvalidEnumError {
    /// Name of the enum.
    pub ty: &'static str,
    /// Read value that is outside of allowed enum values. Needs to be [`i64`] in order to hold all [`u32`] and [`i32`] values.
    pub value: i64,
}

impl InvalidEnumError {
    pub(crate) const fn new(ty: &'static str, value: i64) -> Self {
        Self { ty, value }
    }
}

impl Display for InvalidEnumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Enum '{}' has invalid enumerator value '{}'",
            self.ty, self.value,
        )
    }
}

impl Error for InvalidEnumError {}

impl From<InvalidEnumError> for DbcError {
    fn from(i: InvalidEnumError) -> Self {
        Self::InvalidEnum(i)
    }
}
