use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum RegenError {
    OutOfBounds,
}

impl Display for RegenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegenError::OutOfBounds => write!(f, "Out of map bounds"),
        }
    }
}

impl Error for RegenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
