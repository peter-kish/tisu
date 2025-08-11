use tiled::Loader;

use crate::{
    map::Map,
    map_importer::{LoadResult, MapImporter},
    tisu_error::TisuError,
};

pub struct TiledMapImporter {}

impl TiledMapImporter {
    fn load_finite_tile_layer(
        layer: &tiled::FiniteTileLayer,
    ) -> Result<Map<Option<u32>>, TisuError> {
        let mut map = Map::<Option<u32>>::new((layer.width(), layer.height()).into());
        for x in 0..layer.width() {
            for y in 0..layer.height() {
                if let Some(tile) = layer.get_tile(x as i32, y as i32) {
                    map.set((x, y).into(), Some(tile.id()))?;
                } else {
                    map.set((x, y).into(), None)?;
                }
            }
        }

        Ok(map)
    }
}

impl MapImporter for TiledMapImporter {
    type TileType = Option<u32>;

    fn load(file: impl AsRef<std::path::Path>) -> Result<LoadResult<Self::TileType>, TisuError> {
        let mut loader = Loader::new();
        let tmx_map = loader
            .load_tmx_map(file)
            .map_err(|_| TisuError::InvalidArgument)?;

        let mut result = LoadResult::<Self::TileType> {
            map_layers: vec![],
            tileset_path: tmx_map.tilesets()[0].source.clone(),
        };
        for layer in tmx_map.layers() {
            if let tiled::LayerType::Tiles(tiled::TileLayer::Finite(finite_tile_layer)) =
                layer.layer_type()
            {
                let map = Self::load_finite_tile_layer(&finite_tile_layer)?;

                result.map_layers.push(map);
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let result = TiledMapImporter::load(
            format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "data/3x3.tmx").as_str(),
        );

        assert!(result.is_ok());
        let load_result = result.unwrap();
        assert_eq!(load_result.map_layers.len(), 1);
        assert_eq!(load_result.map_layers[0].size(), (3, 3).into());
        assert_eq!(load_result.map_layers[0].get((0, 0).into()).unwrap(), &None);
        assert_eq!(
            load_result.map_layers[0].get((1, 1).into()).unwrap(),
            &Some(3)
        );
    }

    // TODO: test_load_failure
}
