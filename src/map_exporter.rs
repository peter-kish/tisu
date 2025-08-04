use std::path::Path;

use crate::map::Map;
use crate::tisu_error::TisuError;
use crate::vector2::Vector2u;

pub trait MapExporter {
    fn save(
        file: impl AsRef<Path>,
        map: &Map<Option<u32>>,
        tile_size: Vector2u,
        tileset_path: impl AsRef<Path>,
    ) -> Result<(), TisuError>;
}
