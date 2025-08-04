use std::fs::File;

use xml::{common::XmlVersion, writer::XmlEvent, EmitterConfig};

use crate::exporter::Exporter;

pub struct TiledExporter {}

impl Exporter for TiledExporter {
    // TODO: Test
    fn save(
        file: impl AsRef<std::path::Path>,
        map: &crate::map::Map<Option<u32>>,
        tile_size: crate::vector2::Vector2u,
        tileset_path: impl AsRef<std::path::Path>,
    ) -> Result<(), crate::tisu_error::TisuError> {
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
