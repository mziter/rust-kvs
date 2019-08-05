//! Error module to wrap KvsError and results
use failure::Fail;
use std::io;

/// Error type for KVS
#[derive(Fail, Debug)]
pub enum KvsError {
    /// Error due to std::IO failure
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    /// Error using serde for serialization/deserialization
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::error::Error),

    /// Error when key is not found
    #[fail(display = "Key not found")]
    KeyNotFound,

    /// Error what somehow bad data is read
    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

impl From<serde_json::error::Error> for KvsError {
    fn from(err: serde_json::error::Error) -> KvsError {
        KvsError::Serde(err)
    }
}

/// Maps result type to include Error
pub type Result<T> = std::result::Result<T, KvsError>;
