use tiled::Loader;

use crate::{
    filter::{Filter, FilterCollection, FilterProperties},
    filter_importer::FilterImporter,
    map_importer::MapImporter,
    map_segmenter,
    tiled_map_importer::TiledMapImporter,
    tiled_tile::TiledTile,
    tisu_error::TisuError,
    vector2::Vector2,
};

fn load_layer_properties(
    file: impl AsRef<std::path::Path>,
) -> Result<Vec<FilterProperties>, TisuError> {
    let mut loader = Loader::new();
    let tmx_map = loader
        .load_tmx_map(file)
        .map_err(|_| TisuError::InvalidArgument)?;

    let mut result = vec![];
    // Collect the layers into a Vec to traverse it in reverse order
    for layer in tmx_map.layers().collect::<Vec<_>>().iter().rev() {
        process_layer(layer, &tiled::Properties::new(), &mut result);
    }
    Ok(result)
}

fn process_layer(
    layer: &tiled::Layer,
    parent_properties: &tiled::Properties,
    result: &mut Vec<FilterProperties>,
) {
    let properties = override_properties(parent_properties, &layer.properties);
    match layer.layer_type() {
        tiled::LayerType::Tiles(_) => {
            let mut filter_properties = FilterProperties::from(&properties);
            if !layer.visible {
                filter_properties.ignore = true;
            }
            result.push(filter_properties);
        }
        tiled::LayerType::Group(group) => {
            for child_layer in group.layers().collect::<Vec<_>>().iter().rev() {
                process_layer(child_layer, &properties, result);
            }
        }
        _ => (),
    }
}

fn override_properties(
    parent_properties: &tiled::Properties,
    child_properties: &tiled::Properties,
) -> tiled::Properties {
    let mut result = parent_properties.clone();
    for (key, value) in child_properties {
        result.insert(key.clone(), value.clone());
    }
    result
}

pub struct TiledFilterImporter;

impl FilterImporter for TiledFilterImporter {
    type TileType = TiledTile;

    fn load(
        file: impl AsRef<std::path::Path>,
    ) -> Result<Vec<FilterCollection<Self::TileType>>, TisuError> {
        let load_result = TiledMapImporter::load(&file)?;
        let layer_properties = load_layer_properties(&file)?;

        if load_result.map_layers.len() != layer_properties.len() {
            return Err(TisuError::Unexpected);
        }

        let mut filter_collections = Vec::<FilterCollection<Self::TileType>>::new();
        for (layer, properties) in load_result.map_layers.iter().zip(layer_properties.iter()) {
            let mut filter_collection =
                FilterCollection::<Self::TileType>::new_with_properties(&[], properties.clone());
            let segments = map_segmenter::extract_segments(layer, &TiledTile::default());
            if !segments.is_empty() {
                let mut idx = 0;
                let mut wildcard = TiledTile::default();

                if segments.len() % 2 != 0 {
                    // Try to interpret the first segment as a wildcard
                    if segments[0].size() == Vector2::one() {
                        wildcard = layer.get(segments[0].position())?.clone();
                    }
                    idx = 1;
                }

                while idx < segments.len() - 1 {
                    let pattern_rect = segments[idx];
                    let substitute_rect = segments[idx + 1];
                    let pattern = layer.extract_segment(pattern_rect)?;
                    let substitute = layer.extract_segment(substitute_rect)?;
                    let filter = Filter::new_with_properties(
                        pattern,
                        substitute,
                        wildcard.clone(),
                        properties.clone(),
                    )?;
                    filter_collection.push(filter);
                    idx += 2;
                }
            }
            filter_collections.push(filter_collection);
        }
        Ok(filter_collections)
    }
}

#[cfg(test)]
mod tests {
    use tiled::PropertyValue;

    use crate::{filter::PatternMatching, map::Map};

    use super::*;

    fn create_tiled_map<const N: usize, const M: usize>(data: [[u32; N]; M]) -> Map<TiledTile> {
        Map::<TiledTile>::from_data(data.map(|x| {
            x.map(|x| TiledTile {
                index: Some(x),
                ..Default::default()
            })
        }))
        .unwrap()
    }

    #[test]
    fn test_override_different_properties() {
        let parent_properties: tiled::Properties =
            [(String::from("prop1"), PropertyValue::IntValue(1))].into();
        let child_properties: tiled::Properties =
            [(String::from("prop2"), PropertyValue::IntValue(2))].into();

        let result = override_properties(&parent_properties, &child_properties);

        assert_eq!(result.len(), 2);
        assert!(result.contains_key("prop1"));
        assert_eq!(result.get("prop1").unwrap(), &PropertyValue::IntValue(1));
        assert!(result.contains_key("prop2"));
        assert_eq!(result.get("prop2").unwrap(), &PropertyValue::IntValue(2));
    }

    #[test]
    fn test_override_same_property() {
        let parent_properties: tiled::Properties =
            [(String::from("prop1"), PropertyValue::IntValue(1))].into();
        let child_properties: tiled::Properties =
            [(String::from("prop1"), PropertyValue::IntValue(2))].into();

        let result = override_properties(&parent_properties, &child_properties);

        assert_eq!(result.len(), 1);
        assert!(result.contains_key("prop1"));
        assert_eq!(result.get("prop1").unwrap(), &PropertyValue::IntValue(2));
    }

    #[test]
    fn test_load_layer_properties() {
        let result = load_layer_properties(
            format!(
                "{}/data/test_apply_filter_collection_pattern_matching/filter_collection.tmx",
                env!("CARGO_MANIFEST_DIR"),
            )
            .as_str(),
        );

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0],
            FilterProperties {
                pattern_matching: PatternMatching::Destination,
                ..Default::default()
            }
        );
        assert_eq!(result[1], FilterProperties::default());
    }

    #[test]
    fn test_load() {
        let wildcard = TiledTile {
            index: Some(4),
            ..Default::default()
        };

        let pattern = create_tiled_map([[0, 1]]);
        let substitute = create_tiled_map([[1, 1]]);
        let filter1 = Filter::new(pattern, substitute, wildcard.clone()).unwrap();

        let pattern = create_tiled_map([[2, 2], [2, 2]]);
        let substitute = create_tiled_map([[4, 3], [4, 4]]);
        let filter2 = Filter::new(pattern, substitute, wildcard.clone()).unwrap();

        let pattern = create_tiled_map([[3, 4, 3]]);
        let substitute = create_tiled_map([[0, 0, 0]]);
        let filter3 = Filter::new(pattern, substitute, wildcard.clone()).unwrap();

        let filter_collections = TiledFilterImporter::load(
            format!(
                "{}/data/test_apply_filter_collection/filter_collection.tmx",
                env!("CARGO_MANIFEST_DIR"),
            )
            .as_str(),
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
