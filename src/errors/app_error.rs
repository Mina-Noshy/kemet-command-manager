use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(sled::Error),
    SerializationError(bincode::Error),
    IoError(std::io::Error),
}

impl From<sled::Error> for AppError {
    fn from(err: sled::Error) -> Self {
        AppError::DatabaseError(err)
    }
}

impl From<bincode::Error> for AppError {
    fn from(err: bincode::Error) -> Self {
        AppError::SerializationError(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            AppError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}
