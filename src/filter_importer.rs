use std::path::Path;

use crate::{filter::FilterCollection, tisu_error::TisuError};

pub trait FilterImporter {
    type TileType;

    fn load(file: impl AsRef<Path>) -> Result<Vec<FilterCollection<Self::TileType>>, TisuError>;
}
