use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TisuError {
    OutOfBounds,
    InvalidArgument,
    InvalidMapSize,
    NotFound,
    Unexpected,
}

impl Display for TisuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TisuError::OutOfBounds => write!(f, "Out of map bounds"),
            TisuError::InvalidArgument => write!(f, "Invalid argument"),
            TisuError::InvalidMapSize => write!(f, "Invalid map size"),
            TisuError::NotFound => write!(f, "Not found"),
            TisuError::Unexpected => write!(f, "Unexpected error"),
        }
    }
}

impl Error for TisuError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
