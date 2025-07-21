use std::path::Path;

use crate::filter::{Filter, FilterCollection};
use crate::map_segmenter;
use crate::tiled_map_loader::TiledMapLoader;
use crate::tisu_error::TisuError;

pub struct TiledFilterLoader {}

impl TiledFilterLoader {
    pub fn load(
        file: impl AsRef<Path>,
        wildcard: Option<u32>,
    ) -> Result<FilterCollection<Option<u32>>, TisuError> {
        let load_result = TiledMapLoader::load(file)?;
        let mut filter_collection = FilterCollection::<Option<u32>>::default();
        for layer in &load_result.map_layers {
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
                        load_result.get_properties_for_rects(pattern_rect, substitute_rect)?,
                    )?;
                    filter_collection.push(filter);
                    idx += 2;
                }
            }
        }
        Ok(filter_collection)
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

        let filter_collection = TiledFilterLoader::load(
            format!(
                "{}/data/test_apply_filter_collection/filter_collection.tmx",
                env!("CARGO_MANIFEST_DIR"),
            )
            .as_str(),
            Some(4),
        );

        assert!(filter_collection.is_ok());
        let filters = &filter_collection.unwrap().filters;
        assert_eq!(filters[0], filter1);
        assert_eq!(filters[1], filter2);
        assert_eq!(filters[2], filter3);
    }

    // TODO: test_load_failure
}
