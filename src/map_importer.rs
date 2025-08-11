use std::path::{Path, PathBuf};

use crate::map::Map;
use crate::tisu_error::TisuError;

pub struct LoadResult<T> {
    pub map_layers: Vec<Map<T>>,
    pub tileset_path: PathBuf,
}

pub trait MapImporter {
    type TileType;

    fn load(file: impl AsRef<Path>) -> Result<LoadResult<Self::TileType>, TisuError>;
}
