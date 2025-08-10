use tiled::Loader;

use crate::{
    filter::{Filter, FilterCollection, FilterProperties},
    filter_importer::FilterImporter,
    map_importer::MapImporter,
    map_segmenter,
    tiled_map_importer::TiledMapImporter,
    tisu_error::TisuError,
};

fn load_layer_properties(
    file: impl AsRef<std::path::Path>,
) -> Result<Vec<FilterProperties>, TisuError> {
    let mut loader = Loader::new();
    let tmx_map = loader
        .load_tmx_map(file)
        .map_err(|_| TisuError::InvalidArgument)?;

    let mut result = vec![];
    for layer in tmx_map.layers() {
        if let tiled::LayerType::Tiles(_) = layer.layer_type() {
            result.push(FilterProperties::from(&layer.properties));
        }
    }
    Ok(result)
}

pub struct TiledFilterImporter;

impl FilterImporter for TiledFilterImporter {
    fn load(
        file: impl AsRef<std::path::Path>,
        wildcard: Option<u32>,
    ) -> Result<Vec<FilterCollection<Option<u32>>>, TisuError> {
        let load_result = TiledMapImporter::load(&file)?;
        let layer_properties = load_layer_properties(&file)?;

        if load_result.map_layers.len() != layer_properties.len() {
            return Err(TisuError::Unexpected);
        }

        let mut filter_collections = Vec::<FilterCollection<Option<u32>>>::new();
        for (layer, properties) in load_result.map_layers.iter().zip(layer_properties.iter()) {
            let mut filter_collection =
                FilterCollection::<Option<u32>>::new_with_properties(&[], properties.clone());
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
