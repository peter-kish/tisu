use tiled::Loader;

use crate::{
    filter::{Filter, FilterCollection, FilterProperties},
    filter_importer::FilterImporter,
    map_importer::MapImporter,
    map_segmenter,
    rect2::Rect2u,
    tiled_map_importer::TiledMapImporter,
    tisu_error::TisuError,
    vector2::Vector2u,
};

pub struct PropertyRect {
    pub rect: Rect2u,
    pub properties: FilterProperties,
}

pub struct PropertyLayer {
    pub property_rects: Vec<PropertyRect>,
}

impl PropertyLayer {
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

fn load_property_layers(
    file: impl AsRef<std::path::Path>,
) -> Result<Vec<PropertyLayer>, TisuError> {
    let mut property_layers = vec![];
    let mut loader = Loader::new();
    let tmx_map = loader
        .load_tmx_map(file)
        .map_err(|_| TisuError::InvalidArgument)?;
    let tile_size = Vector2u::new(tmx_map.tile_width, tmx_map.tile_height);
    for layer in tmx_map.layers() {
        if let tiled::LayerType::Objects(object_layer) = layer.layer_type() {
            let property_layer = load_object_layer(&object_layer, tile_size)?;
            property_layers.push(property_layer);
        }
    }
    Ok(property_layers)
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
            let pos_converted = Vector2u::new(position.x / tile_size.x, position.y / tile_size.y);
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

pub fn get_properties_for_rects(
    property_layers: &Vec<PropertyLayer>,
    rect1: Rect2u,
    rect2: Rect2u,
) -> Result<FilterProperties, TisuError> {
    for property_layer in property_layers {
        if let Some(properties) = property_layer.get_properties_for_rects(rect1, rect2) {
            return Ok(properties);
        }
    }
    Ok(FilterProperties::default())
}

pub struct TiledFilterImporter;

impl FilterImporter for TiledFilterImporter {
    fn load(
        file: impl AsRef<std::path::Path>,
        wildcard: Option<u32>,
    ) -> Result<Vec<FilterCollection<Option<u32>>>, crate::tisu_error::TisuError> {
        let load_result = TiledMapImporter::load(&file)?;
        let property_layers = load_property_layers(&file)?;
        let mut filter_colletions = Vec::<FilterCollection<Option<u32>>>::new();
        for layer in &load_result.map_layers {
            let mut filter_collection = FilterCollection::<Option<u32>>::default();
            let segments = map_segmenter::extract_segments(&layer.map, &None);
            if !segments.is_empty() {
                let mut idx = 0;
                while idx < segments.len() - 1 {
                    let pattern_rect = segments[idx];
                    let substitute_rect = segments[idx + 1];
                    let pattern = layer.map.extract_segment(pattern_rect)?;
                    let substitute = layer.map.extract_segment(substitute_rect)?;
                    let filter = Filter::new_with_properties(
                        pattern,
                        substitute,
                        wildcard,
                        get_properties_for_rects(&property_layers, pattern_rect, substitute_rect)?,
                    )?;
                    filter_collection.push(filter);
                    idx += 2;
                }
            }
            filter_colletions.push(filter_collection);
        }
        Ok(filter_colletions)
    }
}

#[cfg(test)]
mod tests {
    use crate::map::Map;

    use super::*;

    #[test]
    fn test_load() {
        let pattern = Map::<Option<u32>>::from_data([[Some(0), Some(1)]]).unwrap();
        let substitute = Map::<Option<u32>>::from_data([[Some(1), Some(1)]]).unwrap();
        let filter1 = Filter::new(pattern, substitute, Some(4)).unwrap();

        let pattern =
            Map::<Option<u32>>::from_data([[Some(2), Some(2)], [Some(2), Some(2)]]).unwrap();
        let substitute =
            Map::<Option<u32>>::from_data([[Some(2), Some(3)], [Some(2), Some(2)]]).unwrap();
        let filter2 = Filter::new(pattern, substitute, Some(4)).unwrap();

        let pattern = Map::<Option<u32>>::from_data([[Some(3), Some(4), Some(3)]]).unwrap();
        let substitute = Map::<Option<u32>>::from_data([[Some(0), Some(0), Some(0)]]).unwrap();
        let filter3 = Filter::new(pattern, substitute, Some(4)).unwrap();

        let filter_collections = TiledFilterImporter::load(
            format!(
                "{}/data/test_apply_filter_collection/filter_collection.tmx",
                env!("CARGO_MANIFEST_DIR"),
            )
            .as_str(),
            Some(4),
        );

        assert!(filter_collections.is_ok());
        let filter_collections = &filter_collections.unwrap();
        assert_eq!(filter_collections.len(), 1);
        let filters = &filter_collections[0].filters;
        assert_eq!(filters[0], filter1);
        assert_eq!(filters[1], filter2);
        assert_eq!(filters[2], filter3);
    }

    // TODO: test_load_failure
}
