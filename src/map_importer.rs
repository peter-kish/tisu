use std::path::{Path, PathBuf};

use crate::map::Map;
use crate::tisu_error::TisuError;

pub struct MapLayer {
    pub map: Map<Option<u32>>,
}

pub struct LoadResult {
    pub map_layers: Vec<MapLayer>,
    pub tileset_path: PathBuf,
}

pub trait MapImporter {
    fn load(file: impl AsRef<Path>) -> Result<LoadResult, TisuError>;
}
