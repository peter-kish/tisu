use std::fs::File;
use std::path::{Path, PathBuf};

use crate::filter::FilterProperties;
use crate::map::Map;
use crate::rect2::Rect2u;
use crate::tisu_error::TisuError;
use crate::vector2::Vector2u;
use tiled::Loader;
use xml::common::XmlVersion;
use xml::writer::XmlEvent;
use xml::EmitterConfig;

pub struct TiledMapLayer {
    pub map: Map<Option<u32>>,
}

pub struct TiledPropertyRect {
    pub rect: Rect2u,
    pub properties: FilterProperties,
}

pub struct TiledPropertyLayer {
    pub property_rects: Vec<TiledPropertyRect>,
}

impl TiledPropertyLayer {
    pub fn get_properties_for_rects(
        &self,
        rect1: Rect2u,
        rect2: Rect2u,
    ) -> Option<FilterProperties> {
        for property_rect in &self.property_rects {
            if property_rect.rect.contains_rect(&rect1) && property_rect.rect.contains_rect(&rect2)
            {
                return Some(property_rect.properties.clone());
            }
        }

        None
    }
}

pub struct TiledLoadResult {
    pub map_layers: Vec<TiledMapLayer>,
    pub property_layers: Vec<TiledPropertyLayer>,
    pub tileset_path: PathBuf,
}

impl TiledLoadResult {
    pub fn get_properties_for_rects(
        &self,
        rect1: Rect2u,
        rect2: Rect2u,
    ) -> Result<FilterProperties, TisuError> {
        for property_layer in &self.property_layers {
            if let Some(properties) = property_layer.get_properties_for_rects(rect1, rect2) {
                return Ok(properties);
            }
        }
        Ok(FilterProperties::default())
    }
}

pub struct TiledMapLoader {}

impl TiledMapLoader {
    pub fn load(file: impl AsRef<Path>) -> Result<TiledLoadResult, TisuError> {
        let mut loader = Loader::new();
        let tmx_map = loader
            .load_tmx_map(file)
            .map_err(|_| TisuError::InvalidArgument)?;

        let mut result = TiledLoadResult {
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

                result.map_layers.push(TiledMapLayer { map });
            }

            if let tiled::LayerType::Objects(object_layer) = layer.layer_type() {
                let property_layer = Self::load_object_layer(&object_layer, tile_size)?;
                result.property_layers.push(property_layer);
            }
        }
        Ok(result)
    }

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
    ) -> Result<TiledPropertyLayer, TisuError> {
        let mut result = TiledPropertyLayer {
            property_rects: vec![],
        };
        for object in layer.objects() {
            if let tiled::ObjectShape::Rect { width, height } = object.shape {
                let position = Vector2u::new(object.x.round() as u32, object.y.round() as u32);
                let size = Vector2u::new(width.round() as u32, height.round() as u32);
                let pos_converted =
                    Vector2u::new(position.x / tile_size.x, position.y / tile_size.y);
                let size_converted = Vector2u::new(size.x / tile_size.x, size.y / tile_size.y);

                let property_rect = TiledPropertyRect {
                    rect: Rect2u::new(pos_converted, size_converted)?,
                    properties: (&object.properties).into(),
                };
                result.property_rects.push(property_rect);
            }
        }
        Ok(result)
    }

    pub fn save(
        file: impl AsRef<Path>,
        map: &Map<Option<u32>>,
        tile_size: Vector2u,
        tileset_path: impl AsRef<Path>,
    ) -> Result<(), TisuError> {
        let target = File::create(file).expect("Failed to create file");
        let mut writer = EmitterConfig::new()
            .perform_indent(true)
            .create_writer(target);

        let event = XmlEvent::StartDocument {
            version: XmlVersion::Version10,
            encoding: "UTF-8".into(),
            standalone: None,
        };
        writer
            .write(event)
            .expect("Failed to write XML document header");

        let map_width_str = map.size().x.to_string();
        let map_height_str = map.size().y.to_string();
        let tile_width_str = tile_size.x.to_string();
        let tile_height_str = tile_size.y.to_string();
        let event = XmlEvent::start_element("map")
            .attr("version", "1.10")
            .attr("tiledversion", "1.11.0")
            .attr("orientation", "orthogonal")
            .attr("renderorder", "right-down")
            .attr("width", map_width_str.as_str())
            .attr("height", map_height_str.as_str())
            .attr("tilewidth", tile_width_str.as_str())
            .attr("tileheight", tile_height_str.as_str())
            .attr("infinite", "0")
            .attr("nextlayerid", "2")
            .attr("nextobjectid", "1");
        writer.write(event).expect("Failed to start 'map' element");

        let tileset_str = tileset_path.as_ref().display().to_string();
        let event = XmlEvent::start_element("tileset")
            .attr("firstgid", "1")
            .attr("source", &tileset_str);
        writer
            .write(event)
            .expect("Failed to start 'tileset' element");

        let event = XmlEvent::end_element();
        writer
            .write(event)
            .expect("Failed to end 'tileset' element");

        let event = XmlEvent::start_element("layer")
            .attr("id", "1")
            .attr("name", "Tile Layer 1")
            .attr("width", map_width_str.as_str())
            .attr("height", map_height_str.as_str());
        writer
            .write(event)
            .expect("Failed to start 'layer' element");

        let event = XmlEvent::start_element("data").attr("encoding", "csv");
        writer.write(event).expect("Failed to start 'data' element");

        let data: Vec<String> = map
            .data()
            .iter()
            .map(|input: &Option<u32>| {
                match input {
                    Some(i) => i + 1,
                    None => 0,
                }
                .to_string()
            })
            .collect();
        let data_str = data.as_slice().join(", ");
        let event = XmlEvent::characters(&data_str);
        writer.write(event).expect("Failed to write data");

        let event = XmlEvent::end_element();
        writer.write(event).expect("Failed to end 'data' element");

        let event = XmlEvent::end_element();
        writer.write(event).expect("Failed to end 'layer' element");

        let event = XmlEvent::end_element();
        writer.write(event).expect("Failed to end 'map' element");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let result = TiledMapLoader::load(
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
    // TODO: test_save
}
