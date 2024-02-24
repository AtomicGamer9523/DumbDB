use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    DBCreateError,
    DBOpenError,
    DBCloseError,
    DBReadError,
    DBWriteError,
    DBRestoreError,
    DBBackupError,
    ParsingError(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            DBCreateError => write!(f, "Failed to create database file"),
            DBOpenError => write!(f, "Failed to open database file"),
            DBCloseError => write!(f, "Failed to close database file"),
            DBReadError => write!(f, "Failed to read from database file"),
            DBWriteError => write!(f, "Failed to write to database file"),
            DBRestoreError => write!(f, "Failed to restore database from backup"),
            DBBackupError => write!(f, "Failed to backup database"),
            ParsingError(c) => write!(f, "Failed to parse data: {c}"),
        }
    }
}

// impl std::error::Error for Error {}
pub type Result<T = (), E = Error> = R<T, E>;
