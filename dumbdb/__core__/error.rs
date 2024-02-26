use std::io::Error as IoError;
use std::io::ErrorKind as K;
use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DumbError {
    DBCreateError,
    DBOpenError,
    DBCloseError,
    DBReadError,
    DBWriteError,
    DBRestoreError,
    DBBackupError,
    DBAlreadyInitialized,
    DBNotInitialized,
    ParsingError(&'static str),
}

impl fmt::Display for DumbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DumbError::*;
        match self {
            DBCreateError => write!(f, "Failed to create database file"),
            DBOpenError => write!(f, "Failed to open database file"),
            DBCloseError => write!(f, "Failed to close database file"),
            DBReadError => write!(f, "Failed to read from database file"),
            DBWriteError => write!(f, "Failed to write to database file"),
            DBRestoreError => write!(f, "Failed to restore database from backup"),
            DBBackupError => write!(f, "Failed to backup database"),
            DBAlreadyInitialized => write!(f, "Database already initialized"),
            DBNotInitialized => write!(f, "Database not initialized"),
            ParsingError(c) => write!(f, "Failed to parse data: {c}"),
        }
    }
}

impl From<IoError> for DumbError {
    fn from(e: IoError) -> Self {
        match e.kind() {
            K::NotFound => DumbError::DBNotInitialized,
            K::AlreadyExists => DumbError::DBAlreadyInitialized,
            _ => DumbError::DBReadError,
        }
    }
}

impl std::error::Error for DumbError {}
pub type DumbResult<T = (), E = DumbError> = R<T, E>;
