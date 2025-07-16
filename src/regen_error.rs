use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RegenError {
    OutOfBounds,
    InvalidArgument,
    InvalidMapSize,
    NotFound,
    Unexpected,
}

impl Display for RegenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegenError::OutOfBounds => write!(f, "Out of map bounds"),
            RegenError::InvalidArgument => write!(f, "Invalid argument"),
            RegenError::InvalidMapSize => write!(f, "Invalid map size"),
            RegenError::NotFound => write!(f, "Not found"),
            RegenError::Unexpected => write!(f, "Unexpected error"),
        }
    }
}

impl Error for RegenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
