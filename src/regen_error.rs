use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RegenError {
    OutOfBounds,
    InvalidArgument,
}

impl Display for RegenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegenError::OutOfBounds => write!(f, "Out of map bounds"),
            RegenError::InvalidArgument => write!(f, "Invalid argument"),
        }
    }
}

impl Error for RegenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
