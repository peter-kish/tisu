use tiled::Loader;

use crate::{
    map::Map,
    map_importer::{LoadResult, MapImporter, MapLayer, PropertyLayer, PropertyRect},
    rect2::Rect2u,
    tisu_error::TisuError,
    vector2::Vector2u,
};

pub struct TiledImporter {}

impl TiledImporter {
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

    fn load_object_layer(
        layer: &tiled::ObjectLayer,
        tile_size: Vector2u,
    ) -> Result<PropertyLayer, TisuError> {
        let mut result = PropertyLayer {
            property_rects: vec![],
        };
        for object in layer.objects() {
            if let tiled::ObjectShape::Rect { width, height } = object.shape {
                let position = Vector2u::new(object.x.round() as u32, object.y.round() as u32);
                let size = Vector2u::new(width.round() as u32, height.round() as u32);
                let pos_converted =
                    Vector2u::new(position.x / tile_size.x, position.y / tile_size.y);
                let size_converted = Vector2u::new(size.x / tile_size.x, size.y / tile_size.y);

                let property_rect = PropertyRect {
                    rect: Rect2u::new(pos_converted, size_converted)?,
                    properties: (&object.properties).into(),
                };
                result.property_rects.push(property_rect);
            }
        }
        Ok(result)
    }
}

impl MapImporter for TiledImporter {
    fn load(file: impl AsRef<std::path::Path>) -> Result<LoadResult, TisuError> {
        let mut loader = Loader::new();
        let tmx_map = loader
            .load_tmx_map(file)
            .map_err(|_| TisuError::InvalidArgument)?;

        let mut result = LoadResult {
            map_layers: vec![],
            property_layers: vec![],
            tileset_path: tmx_map.tilesets()[0].source.clone(),
        };
        let tile_size = Vector2u::new(tmx_map.tile_width, tmx_map.tile_height);
        for layer in tmx_map.layers() {
            if let tiled::LayerType::Tiles(tiled::TileLayer::Finite(finite_tile_layer)) =
                layer.layer_type()
            {
                let map = Self::load_finite_tile_layer(&finite_tile_layer)?;

                result.map_layers.push(MapLayer { map });
            }

            if let tiled::LayerType::Objects(object_layer) = layer.layer_type() {
                let property_layer = Self::load_object_layer(&object_layer, tile_size)?;
                result.property_layers.push(property_layer);
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
        let result = TiledImporter::load(
            format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "data/3x3.tmx").as_str(),
        );

        assert!(result.is_ok());
        let load_result = result.unwrap();
        assert_eq!(load_result.map_layers.len(), 1);
        assert_eq!(load_result.map_layers[0].map.size(), (3, 3).into());
        assert_eq!(
            load_result.map_layers[0].map.get((0, 0).into()).unwrap(),
            &None
        );
        assert_eq!(
            load_result.map_layers[0].map.get((1, 1).into()).unwrap(),
            &Some(3)
        );
    }

    // TODO: test_load_failure
}
