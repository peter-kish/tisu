use rand::Rng;

use tiled::{Properties, PropertyValue};

use crate::map::Map;
use crate::map_segmenter;
use crate::regen_error::RegenError;
use crate::tiled_map_converter::TiledMapConverter;
use crate::vector2::Vector2u;

#[derive(Clone, PartialEq, Debug)]
pub struct FilterProperties {
    probability: f32,
}

impl From<&Properties> for FilterProperties {
    fn from(value: &Properties) -> Self {
        let probability = match value.get("probability") {
            Some(PropertyValue::FloatValue(p)) => p,
            _ => &1.0,
        };

        Self {
            probability: *probability,
        }
    }
}

impl Default for FilterProperties {
    fn default() -> Self {
        Self { probability: 1.0 }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Filter<T> {
    pattern: Map<T>,
    substitute: Map<T>,
    wildcard: T,
    properties: FilterProperties,
}

impl<T> Filter<T> {
    pub fn new(pattern: Map<T>, substitute: Map<T>, wildcard: T) -> Result<Self, RegenError> {
        if pattern.get_size() != substitute.get_size() {
            Err(RegenError::InvalidArgument)
        } else {
            Ok(Self {
                pattern,
                substitute,
                wildcard,
                properties: FilterProperties::default(),
            })
        }
    }

    pub fn new_with_properties(
        pattern: Map<T>,
        substitute: Map<T>,
        wildcard: T,
        properties: FilterProperties,
    ) -> Result<Self, RegenError> {
        if pattern.get_size() != substitute.get_size() {
            Err(RegenError::InvalidArgument)
        } else {
            Ok(Self {
                pattern,
                substitute,
                wildcard,
                properties,
            })
        }
    }

    pub fn get_pattern(&self) -> &Map<T> {
        &self.pattern
    }

    pub fn get_substitute(&self) -> &Map<T> {
        &self.substitute
    }

    pub fn pattern_matches(&self, input: &Map<T>, position: Vector2u) -> bool
    where
        T: PartialEq,
    {
        if rand::rng().random_range(0.0..1.0) > self.properties.probability {
            return false;
        }
        for x in 0..self.pattern.get_size().x {
            for y in 0..self.pattern.get_size().y {
                let point = Vector2u::new(x, y);
                if let Ok(input_field) = input.get(position + point) {
                    if let Ok(pattern_field) = self.pattern.get(point) {
                        if !self.fields_match(input_field, pattern_field) {
                            return false;
                        }
                    }
                } else {
                    return false;
                }
            }
        }

        true
    }

    fn fields_match(&self, input_field: &T, pattern_field: &T) -> bool
    where
        T: PartialEq,
    {
        input_field == pattern_field || pattern_field == &self.wildcard
    }

    pub fn substitute(&self, input: &mut Map<T>, position: Vector2u)
    where
        T: Clone + PartialEq,
    {
        for x in 0..self.pattern.get_size().x {
            for y in 0..self.pattern.get_size().y {
                let point = Vector2u::new(x, y);
                if let Ok(substitute_field) = self.substitute.get(point) {
                    self.substitute_field(input, position + point, substitute_field);
                }
            }
        }
    }

    fn substitute_field(&self, input: &mut Map<T>, position: Vector2u, substitute_field: &T)
    where
        T: Clone + PartialEq,
    {
        if substitute_field != &self.wildcard {
            _ = input.set(position, substitute_field.clone());
        }
    }

    pub fn apply(&self, map: &Map<T>) -> Result<Map<T>, RegenError>
    where
        Map<T>: Clone,
        T: Clone + PartialEq,
    {
        if map.get_size().x < self.get_pattern().get_size().x
            || map.get_size().y < self.get_pattern().get_size().y
        {
            Err(RegenError::InvalidArgument)
        } else {
            let mut result = map.clone();
            for x in 0..=map.get_size().x - self.get_pattern().get_size().x {
                for y in 0..=map.get_size().y - self.get_pattern().get_size().y {
                    let point = Vector2u::new(x, y);
                    if self.pattern_matches(map, point) {
                        self.substitute(&mut result, point);
                    }
                }
            }
            Ok(result)
        }
    }
}

#[derive(Default)]
pub struct FilterCollection<T> {
    filters: Vec<Filter<T>>,
}

impl<T> FilterCollection<T> {
    pub fn new(filters: &[Filter<T>]) -> Self
    where
        Filter<T>: Clone,
    {
        Self {
            filters: filters.into(),
        }
    }

    pub fn apply(&self, map: &Map<T>) -> Result<Map<T>, RegenError>
    where
        T: Clone + PartialEq,
    {
        let mut maybe_result: Option<Map<T>> = None;
        for filter in &self.filters {
            maybe_result = match maybe_result {
                Some(result) => Some(filter.apply(&result)?),
                None => Some(filter.apply(map)?),
            };
        }
        maybe_result.ok_or(RegenError::InvalidArgument)
    }

    pub fn push(&mut self, filter: Filter<T>) {
        self.filters.push(filter);
    }
}

pub fn load_tiled_filters(
    file: &str,
    wildcard: Option<u32>,
) -> Result<FilterCollection<Option<u32>>, RegenError> {
    let load_result = TiledMapConverter::load(file)?;
    let mut filter_collection = FilterCollection::<Option<u32>>::default();
    for layer in &load_result.map_layers {
        let segments = map_segmenter::extract_segments(&layer.map, &None);
        if segments.is_empty() || segments.len() % 2 > 0 {
            return Err(RegenError::InvalidArgument);
        } else {
            let mut idx = 0;
            loop {
                if idx >= segments.len() {
                    break;
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor_success() {
        let pattern = Map::<u32>::new((2, 2).into());
        let substitute = Map::<u32>::new((2, 2).into());
        let result = Filter::new(pattern.clone(), substitute.clone(), 42);

        assert!(result.is_ok());
        let filter = result.unwrap();
        assert_eq!(filter.pattern, pattern);
        assert_eq!(filter.substitute, substitute);
    }

    #[test]
    fn test_constructor_failure() {
        let pattern = Map::<u32>::new((2, 2).into());
        let substitute = Map::<u32>::new((3, 2).into());
        let result = Filter::new(pattern, substitute, 42);

        assert_eq!(result.err().unwrap(), RegenError::InvalidArgument);
    }

    #[test]
    fn test_pattern_matches() {
        let map = Map::<u32>::from_data([[1, 0], [0, 1]]).unwrap();
        let pattern = Map::<u32>::from_data([[1, 0]]).unwrap();
        let substitute = Map::<u32>::from_data([[1, 0]]).unwrap();
        let filter = Filter::new(pattern, substitute, 42).unwrap();

        assert!(filter.pattern_matches(&map, (0, 0).into()));
        assert!(!filter.pattern_matches(&map, (0, 1).into()));
        assert!(!filter.pattern_matches(&map, (1, 1).into()));
    }

    #[test]
    fn test_pattern_match_with_wildcard() {
        let map = Map::<u32>::from_data([[1, 0], [1, 1]]).unwrap();
        let pattern = Map::<u32>::from_data([[1, 2]]).unwrap();
        let substitute = Map::<u32>::from_data([[1, 0]]).unwrap();
        let filter = Filter::new(pattern, substitute, 2).unwrap();

        assert!(filter.pattern_matches(&map, (0, 0).into()));
        assert!(filter.pattern_matches(&map, (0, 1).into()));
        assert!(!filter.pattern_matches(&map, (1, 1).into()));
    }

    #[test]
    fn test_substitute() {
        let mut map = Map::<u32>::from_data([[1, 0], [0, 1]]).unwrap();
        let pattern = Map::<u32>::from_data([[1, 0]]).unwrap();
        let substitute = Map::<u32>::from_data([[1, 0]]).unwrap();
        let filter = Filter::new(pattern, substitute, 42).unwrap();

        filter.substitute(&mut map, (0, 1).into());
        assert_eq!(map.get_data(), [1, 0, 1, 0]);

        filter.substitute(&mut map, (1, 0).into());
        assert_eq!(map.get_data(), [1, 1, 1, 0]);
    }

    #[test]
    fn test_substitute_with_wildcard() {
        let mut map = Map::<u32>::from_data([[1, 0], [0, 1]]).unwrap();
        let pattern = Map::<u32>::from_data([[1, 0]]).unwrap();
        let substitute = Map::<u32>::from_data([[1, 2]]).unwrap();
        let filter = Filter::new(pattern, substitute, 2).unwrap();

        filter.substitute(&mut map, (0, 1).into());
        assert_eq!(map.get_data(), [1, 0, 1, 1]);
    }

    #[test]
    fn test_apply_filter_success() {
        // 1 0 1
        // 1 1 1
        // 1 0 1
        let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
        // 1 0
        let pattern = Map::<u32>::from_data([[1, 0]]).unwrap();
        // 0 1
        let substitute = Map::<u32>::from_data([[0, 1]]).unwrap();
        let filter = Filter::new(pattern, substitute, 42).unwrap();
        // 0 1 1
        // 1 1 1
        // 0 1 1
        let expected_data = [0, 1, 1, 1, 1, 1, 0, 1, 1];

        let result = filter.apply(&map);

        assert!(result.is_ok());
        let result_map = result.unwrap();
        assert_eq!(result_map.get_data(), &expected_data);
        assert_eq!(result_map.get_size(), map.get_size());
    }

    #[test]
    fn test_apply_filter_failure() {
        // 1 0 1
        // 1 1 1
        // 1 0 1
        let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
        // 1 0 0 0
        let pattern = Map::<u32>::from_data([[1, 0, 0, 0]]).unwrap();
        // 0 1 1 1
        let substitute = Map::<u32>::from_data([[0, 1, 1, 1]]).unwrap();
        let filter = Filter::new(pattern, substitute, 42).unwrap();

        let result = filter.apply(&map);

        assert_eq!(result.err().unwrap(), RegenError::InvalidArgument);
    }

    #[test]
    fn test_apply_filter_patter_with_wildcard() {
        // 1 0 1
        // 1 1 1
        // 1 0 1
        let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
        // 1 W 1
        let pattern = Map::<u32>::from_data([[1, 2, 1]]).unwrap();
        // 0 1 0
        let substitute = Map::<u32>::from_data([[0, 1, 0]]).unwrap();
        let filter = Filter::new(pattern, substitute, 2).unwrap();
        // 0 1 0
        // 0 1 0
        // 0 1 0
        let expected_data = [0, 1, 0, 0, 1, 0, 0, 1, 0];

        let result = filter.apply(&map);

        assert!(result.is_ok());
        let result_map = result.unwrap();
        assert_eq!(result_map.get_data(), &expected_data);
        assert_eq!(result_map.get_size(), map.get_size());
    }

    #[test]
    fn test_apply_filter_substitute_with_wildcard() {
        // 1 0 1
        // 1 1 1
        // 1 0 1
        let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
        // 1 0 1
        let pattern = Map::<u32>::from_data([[1, 0, 1]]).unwrap();
        // W 1 W
        let substitute = Map::<u32>::from_data([[2, 1, 2]]).unwrap();
        let filter = Filter::new(pattern, substitute, 2).unwrap();
        // 1 1 1
        // 1 1 1
        // 1 1 1
        let expected_data = [1, 1, 1, 1, 1, 1, 1, 1, 1];

        let result = filter.apply(&map);

        assert!(result.is_ok());
        let result_map = result.unwrap();
        assert_eq!(result_map.get_data(), &expected_data);
        assert_eq!(result_map.get_size(), map.get_size());
    }

    #[test]
    fn test_apply_filter_collection_success() {
        // 1 0 1
        // 1 1 1
        // 1 0 1
        let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
        // 1 0
        let pattern1 = Map::<u32>::from_data([[1, 0]]).unwrap();
        // 0 1
        let substitute1 = Map::<u32>::from_data([[0, 1]]).unwrap();
        let filter1 = Filter::new(pattern1, substitute1, 42).unwrap();
        // 0 1 1
        let pattern2 = Map::<u32>::from_data([[0, 1, 1]]).unwrap();
        // 0 0 0
        let substitute2 = Map::<u32>::from_data([[0, 0, 0]]).unwrap();
        let filter2 = Filter::new(pattern2, substitute2, 42).unwrap();
        let filter_collection = FilterCollection::new(&[filter1, filter2]);
        // 0 0 0
        // 1 1 1
        // 0 0 0
        let expected_data = [0, 0, 0, 1, 1, 1, 0, 0, 0];

        let result = filter_collection.apply(&map);

        assert!(result.is_ok());
        let result_map = result.unwrap();
        assert_eq!(result_map.get_data(), &expected_data);
        assert_eq!(result_map.get_size(), map.get_size());
    }

    #[test]
    fn test_apply_filter_collection_failure() {
        // 1 0 1
        // 1 1 1
        // 1 0 1
        let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
        let filter_collection = FilterCollection::new(&[]);
        let result = filter_collection.apply(&map);

        assert_eq!(result.err().unwrap(), RegenError::InvalidArgument);
    }

    #[test]
    fn test_filter_collection_push() {
        let mut fc = FilterCollection::<u32>::default();
        // 1 0
        let pattern = Map::<u32>::from_data([[1, 0]]).unwrap();
        // 0 1
        let substitute = Map::<u32>::from_data([[0, 1]]).unwrap();
        let filter = Filter::new(pattern, substitute, 42).unwrap();

        fc.push(filter.clone());

        assert_eq!(fc.filters.len(), 1);
        assert_eq!(fc.filters[0], filter);
    }

    #[test]
    fn test_filter_collection_load_tiled_success() {
        let pattern = Map::<Option<u32>>::from_data([[Some(0), Some(1)]]).unwrap();
        let substitute = Map::<Option<u32>>::from_data([[Some(1), Some(1)]]).unwrap();
        let filter1 = Filter::new_with_properties(
            pattern,
            substitute,
            Some(4),
            FilterProperties { probability: 0.0 },
        )
        .unwrap();

        let pattern =
            Map::<Option<u32>>::from_data([[Some(2), Some(2)], [Some(2), Some(2)]]).unwrap();
        let substitute =
            Map::<Option<u32>>::from_data([[Some(2), Some(3)], [Some(2), Some(2)]]).unwrap();
        let filter2 = Filter::new(pattern, substitute, Some(4)).unwrap();

        let pattern = Map::<Option<u32>>::from_data([[Some(3), Some(4), Some(3)]]).unwrap();
        let substitute = Map::<Option<u32>>::from_data([[Some(0), Some(0), Some(0)]]).unwrap();
        let filter3 = Filter::new(pattern, substitute, Some(4)).unwrap();

        let fc = load_tiled_filters(
            format!(
                "{}/{}",
                env!("CARGO_MANIFEST_DIR"),
                "data/filter_collection.tmx"
            )
            .as_str(),
            Some(4),
        );

        assert!(fc.is_ok());
        let filters = &fc.unwrap().filters;
        assert_eq!(filters[0], filter1);
        assert_eq!(filters[1], filter2);
        assert_eq!(filters[2], filter3);
    }
}
