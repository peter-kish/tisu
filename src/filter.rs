use rand::Rng;

use tiled::{Properties, PropertyValue};

use crate::map::Map;
use crate::map_segmenter;
use crate::regen_error::RegenError;
use crate::tiled_map_converter::TiledMapConverter;
use crate::vector2::Vector2u;

#[derive(Clone, PartialEq, Debug, Default)]
pub enum ApplyTo {
    #[default]
    Destination,
    Source,
}

impl TryFrom<&String> for ApplyTo {
    type Error = ();
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "destination" => Ok(ApplyTo::Destination),
            "source" => Ok(ApplyTo::Source),
            _ => Err(()),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct FilterProperties {
    probability: f32,
    apply_to: ApplyTo,
}

impl From<&Properties> for FilterProperties {
    fn from(value: &Properties) -> Self {
        let probability = match value.get("probability") {
            Some(PropertyValue::FloatValue(p)) => *p,
            _ => 1.0,
        };
        let apply_to: ApplyTo = match value.get("apply_to") {
            Some(PropertyValue::StringValue(p)) => p.try_into().unwrap_or(ApplyTo::default()),
            _ => ApplyTo::Destination,
        };

        Self {
            probability,
            apply_to,
        }
    }
}

impl Default for FilterProperties {
    fn default() -> Self {
        Self {
            probability: 1.0,
            apply_to: ApplyTo::Destination,
        }
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
        if pattern.size() != substitute.size() {
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
        if pattern.size() != substitute.size() {
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

    pub fn pattern(&self) -> &Map<T> {
        &self.pattern
    }

    pub fn substitute(&self) -> &Map<T> {
        &self.substitute
    }

    pub fn pattern_matches(&self, input: &Map<T>, position: Vector2u) -> bool
    where
        T: PartialEq,
    {
        if rand::rng().random_range(0.0..1.0) > self.properties.probability {
            return false;
        }
        for x in 0..self.pattern.size().x {
            for y in 0..self.pattern.size().y {
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

    pub fn apply_substitute(&self, input: &mut Map<T>, position: Vector2u)
    where
        T: Clone + PartialEq,
    {
        for x in 0..self.pattern.size().x {
            for y in 0..self.pattern.size().y {
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
        if map.size().x < self.pattern().size().x || map.size().y < self.pattern().size().y {
            Err(RegenError::InvalidArgument)
        } else {
            let mut destination = map.clone();
            for x in 0..=map.size().x - self.pattern().size().x {
                for y in 0..=map.size().y - self.pattern().size().y {
                    let point = Vector2u::new(x, y);
                    match self.properties.apply_to {
                        // TODO: Test different ApplyTo variants
                        ApplyTo::Destination => {
                            if self.pattern_matches(map, point) {
                                self.apply_substitute(&mut destination, point);
                            }
                        }
                        ApplyTo::Source => {
                            if self.pattern_matches(&destination, point) {
                                self.apply_substitute(&mut destination, point);
                            }
                        }
                    }
                }
            }
            Ok(destination)
        }
    }
}

#[derive(Default)]
pub struct FilterCollection<T> {
    pub filters: Vec<Filter<T>>,
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
