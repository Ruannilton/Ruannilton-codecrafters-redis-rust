use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum RedisError {
    UnexpectedToken,
    InvalidCommand(String),
    NoTokenAvailable,
    InvalidArgument,
    LockError,
    InvalidStreamEntryId(String),
    RestoreRDBError,
    RDBDecodeSizeError(u8, u8, u8),
    RDBInvalidSizeModeError(u8, u8, u8),
    RDBInvalidHeader,
    IOError(std::io::Error),
    ParsingError,
    InvalidOpCode,
}

impl Error for RedisError {}

impl Display for RedisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            RedisError::UnexpectedToken => write!(f, "Unexpected token"),
            RedisError::NoTokenAvailable => write!(f, "No token available"),
            RedisError::InvalidArgument => write!(f, "Invalid argument"),
            RedisError::InvalidCommand(cmd) => write!(f, "Invalid command: {}", cmd),
            RedisError::InvalidStreamEntryId(v) => {
                write!(f, "Value provided is not a valid stream entry id: {}", v)
            }
            RedisError::LockError => write!(f, "Failed to lock resource"),
            RedisError::RestoreRDBError => write!(f, "Failed to restore from RDB"),
            RedisError::RDBDecodeSizeError(b, m, r) => {
                write!(
                    f,
                    "Failed to parse bytes do size encoded value. Byte: {}, mode: {}, remain: {}",
                    b, m, r
                )
            }
            RedisError::RDBInvalidSizeModeError(b, m, r) => {
                write!(
                    f,
                    "Modo inválido ao decodificar tamanho. Byte: {}, mode: {}, remain: {}",
                    b, m, r
                )
            }
            RedisError::IOError(err) => err.fmt(f),
            RedisError::RDBInvalidHeader => write!(f, "RDB header is invalid"),
            RedisError::ParsingError => write!(f, "Parsing error"),
            RedisError::InvalidOpCode => write!(f, "Invalid Op Code"),
        }
    }
}
