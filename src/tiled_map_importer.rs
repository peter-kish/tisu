use tiled::Loader;

use crate::{
    map::Map,
    map_importer::{LoadResult, MapImporter},
    tiled_tile::TiledTile,
    tisu_error::TisuError,
};

pub struct TiledMapImporter {}

impl TiledMapImporter {
    fn load_finite_tile_layer(
        layer: &tiled::FiniteTileLayer,
    ) -> Result<Map<<TiledMapImporter as MapImporter>::TileType>, TisuError> {
        let mut map = Map::<<TiledMapImporter as MapImporter>::TileType>::new(
            (layer.width(), layer.height()).into(),
        );
        for x in 0..layer.width() {
            for y in 0..layer.height() {
                if let Some(tile) = layer.get_tile(x as i32, y as i32) {
                    map.set(
                        (x, y).into(),
                        TiledTile {
                            index: Some(tile.id()),
                            flip_h: tile.flip_h,
                            flip_v: tile.flip_v,
                            flip_d: tile.flip_d,
                        },
                    )?;
                } else {
                    map.set((x, y).into(), TiledTile::default())?;
                }
            }
        }

        Ok(map)
    }
}

impl MapImporter for TiledMapImporter {
    type TileType = TiledTile;

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
        assert_eq!(
            load_result.map_layers[0].get((0, 0).into()).unwrap(),
            &TiledTile::default()
        );
        assert_eq!(
            load_result.map_layers[0].get((1, 1).into()).unwrap(),
            &TiledTile {
                index: Some(3),
                ..Default::default()
            }
        );
    }

    // TODO: test_load_failure
}
