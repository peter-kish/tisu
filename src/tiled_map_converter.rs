use std::fs::File;

use crate::map::Map;
use crate::regen_error::RegenError;
use crate::vector2::Vector2u;
use tiled::Loader;
use xml::common::XmlVersion;
use xml::writer::XmlEvent;
use xml::EmitterConfig;

pub struct TiledMapConverter {}

impl TiledMapConverter {
    pub fn load(file: &str) -> Result<Vec<Map<Option<u32>>>, RegenError> {
        let mut loader = Loader::new();
        let tmx_map = loader
            .load_tmx_map(file)
            .map_err(|_| RegenError::InvalidArgument)?;

        let mut maps: Vec<Map<Option<u32>>> = vec![];
        for layer in tmx_map.layers() {
            if let tiled::LayerType::Tiles(tiled::TileLayer::Finite(finite)) = layer.layer_type() {
                let mut map = Map::<Option<u32>>::new((finite.width(), finite.height()).into());
                for x in 0..finite.width() {
                    for y in 0..finite.height() {
                        if let Some(tile) = finite.get_tile(x as i32, y as i32) {
                            map.set((x, y).into(), Some(tile.id()))?;
                        } else {
                            map.set((x, y).into(), None)?;
                        }
                    }
                }
                maps.push(map);
            }
        }
        Ok(maps)
    }

    // TODO: Test
    pub fn save(
        file: &str,
        map: &Map<Option<u32>>,
        tile_size: Vector2u,
        tileset_file: &str,
    ) -> Result<(), RegenError> {
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

        let map_width_str = map.get_size().x.to_string();
        let map_height_str = map.get_size().y.to_string();
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

        let event = XmlEvent::start_element("tileset")
            .attr("firstgid", "1")
            .attr("source", tileset_file);
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
            .get_data()
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
        let result = TiledMapConverter::load(
            format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "data/3x3.tmx").as_str(),
        );

        assert!(result.is_ok());
        let maps = result.unwrap();
        assert_eq!(maps.len(), 1);
        assert_eq!(maps[0].get_size(), (3, 3).into());
        assert_eq!(maps[0].get((0, 0).into()).unwrap(), &None);
        assert_eq!(maps[0].get((1, 1).into()).unwrap(), &Some(3));
    }
}
