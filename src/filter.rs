use rand::Rng;

use tiled::{Properties, PropertyValue};

use crate::map::Map;
use crate::tisu_error::TisuError;
use crate::vector2::Vector2u;

/// Filter property that defines where the filter will be applied (source or
/// destination)
#[derive(Clone, PartialEq, Debug, Default)]
pub enum PatternMatching {
    #[default]
    Source,
    Destination,
}

impl TryFrom<&String> for PatternMatching {
    type Error = ();
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "destination" => Ok(PatternMatching::Destination),
            "source" => Ok(PatternMatching::Source),
            _ => Err(()),
        }
    }
}

type ApplicationMap = Map<bool>;

/// Filter properties.
#[derive(Clone, PartialEq, Debug)]
pub struct FilterProperties {
    /// Probability of the filter being applied on each pattern match (clamped
    /// to range [0..1]).
    probability: f32,
    /// Defines where the filter will be applied (source or destination).
    apply_to: PatternMatching,
    /// If true, the filter will be applied only once per field. Once a filter
    /// has been applied to a field, that field will not result in any further
    /// pattern matches.
    only_once: bool,
}

impl From<&Properties> for FilterProperties {
    fn from(value: &Properties) -> Self {
        let probability = match value.get("probability") {
            Some(PropertyValue::FloatValue(p)) => *p,
            _ => 1.0,
        };
        let apply_to: PatternMatching = match value.get("pattern_matching") {
            Some(PropertyValue::StringValue(p)) => {
                p.try_into().unwrap_or(PatternMatching::default())
            }
            _ => PatternMatching::default(),
        };
        let only_once = match value.get("only_once") {
            Some(PropertyValue::BoolValue(p)) => *p,
            _ => false,
        };

        Self {
            probability,
            apply_to,
            only_once,
        }
    }
}

impl Default for FilterProperties {
    fn default() -> Self {
        Self {
            probability: 1.0,
            apply_to: PatternMatching::default(),
            only_once: false,
        }
    }
}

/// Map filter
#[derive(Clone, PartialEq, Debug)]
pub struct Filter<T> {
    /// Defines to which fields the substitute will be applied to.
    pattern: Map<T>,
    /// Applied to fields that match the filter pattern.
    substitute: Map<T>,
    /// Field value that represents a wildcard, which can affect pattern
    /// matching and substitute application.
    wildcard: T,
    /// Filter properties that can affect pattern matching or substitute
    /// application.
    properties: FilterProperties,
}

impl<T> Filter<T> {
    /// Creates a filter with the given pattern map, substitute map and wildcard
    /// value.
    ///
    /// # Errors
    ///
    /// Returns an error if the given pattern and substitute don't have equal
    /// sizes.
    pub fn new(pattern: Map<T>, substitute: Map<T>, wildcard: T) -> Result<Self, TisuError> {
        if pattern.size() != substitute.size() {
            Err(TisuError::InvalidMapSize)
        } else {
            Ok(Self {
                pattern,
                substitute,
                wildcard,
                properties: FilterProperties::default(),
            })
        }
    }

    /// Creates a filter with the given pattern map, substitute map, wildcard
    /// value and filter properties.
    ///
    /// # Errors
    ///
    /// Returns an error if the given pattern and substitute don't have equal
    /// sizes.
    pub fn new_with_properties(
        pattern: Map<T>,
        substitute: Map<T>,
        wildcard: T,
        properties: FilterProperties,
    ) -> Result<Self, TisuError> {
        if pattern.size() != substitute.size() {
            Err(TisuError::InvalidMapSize)
        } else {
            Ok(Self {
                pattern,
                substitute,
                wildcard,
                properties,
            })
        }
    }

    /// Returns the filter pattern map.
    pub fn pattern(&self) -> &Map<T> {
        &self.pattern
    }

    /// Returns the filter substitute map.
    pub fn substitute(&self) -> &Map<T> {
        &self.substitute
    }

    /// Checks if the filter pattern matches at the given position in the given
    /// input map. Optionally, an application map can be used which defines the
    /// fields where the filter has already been applied.
    pub fn pattern_matches(
        &self,
        input: &Map<T>,
        position: Vector2u,
        application_map: &Option<ApplicationMap>,
    ) -> bool
    where
        T: PartialEq,
    {
        if rand::rng().random_range(0.0..1.0) > self.properties.probability {
            return false;
        }
        for x in 0..self.pattern.size().x {
            for y in 0..self.pattern.size().y {
                let point = Vector2u::new(x, y);
                if Self::already_applied(application_map, position + point) {
                    return false;
                }
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

    fn already_applied(application_map: &Option<ApplicationMap>, point: Vector2u) -> bool {
        match application_map {
            None => false,
            Some(application_map) => match application_map.get(point) {
                Err(_) => false,
                Ok(already_applied) => *already_applied,
            },
        }
    }

    fn fields_match(&self, input_field: &T, pattern_field: &T) -> bool
    where
        T: PartialEq,
    {
        input_field == pattern_field || pattern_field == &self.wildcard
    }

    /// Applies the filter substitute to the given input map at the given
    /// position. Optionally, an application map can be used to mark the fields
    /// where the substitute has been applied.
    pub fn apply_substitute(
        &self,
        input: &mut Map<T>,
        position: Vector2u,
        application_map: &mut Option<ApplicationMap>,
    ) where
        T: Clone + PartialEq,
    {
        for x in 0..self.pattern.size().x {
            for y in 0..self.pattern.size().y {
                let point = Vector2u::new(x, y);
                if let Ok(substitute_field) = self.substitute.get(point) {
                    self.substitute_field(input, position + point, substitute_field);
                    if let Some(application_map) = application_map {
                        _ = application_map.set(position + point, true);
                    }
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

    /// Applies the filter to the given map.
    ///
    /// # Errors
    ///
    /// Returns an error if map size is smaller than that of the pattern or
    /// substitute maps.
    pub fn apply(&self, source: &Map<T>, destination: &mut Map<T>) -> Result<(), TisuError>
    where
        Map<T>: Clone,
        T: Clone + PartialEq,
    {
        if source.size() != destination.size()
            || source.size().x < self.pattern().size().x
            || source.size().y < self.pattern().size().y
        {
            Err(TisuError::InvalidMapSize)
        } else {
            let mut application_map = if self.properties.only_once {
                Some(ApplicationMap::new(source.size()))
            } else {
                None
            };

            for x in 0..=source.size().x - self.pattern().size().x {
                for y in 0..=source.size().y - self.pattern().size().y {
                    let point = Vector2u::new(x, y);
                    match self.properties.apply_to {
                        PatternMatching::Destination => {
                            if self.pattern_matches(destination, point, &application_map) {
                                self.apply_substitute(destination, point, &mut application_map);
                            }
                        }
                        PatternMatching::Source => {
                            if self.pattern_matches(source, point, &application_map) {
                                self.apply_substitute(destination, point, &mut application_map);
                            }
                        }
                    }
                }
            }
            Ok(())
        }
    }
}

/// A collection of map filters
#[derive(Default)]
pub struct FilterCollection<T> {
    /// Vector containing the filters
    pub filters: Vec<Filter<T>>,
    pub properties: FilterProperties,
}

impl<T> FilterCollection<T> {
    /// Creates a filter collection from the given array of filters.
    pub fn new(filters: &[Filter<T>]) -> Self
    where
        Filter<T>: Clone,
    {
        Self {
            filters: filters.into(),
            properties: FilterProperties::default(),
        }
    }

    /// Creates a filter collection from the given array of filters and filter
    /// collection properties.
    pub fn new_with_properties(filters: &[Filter<T>], properties: FilterProperties) -> Self
    where
        Filter<T>: Clone,
    {
        Self {
            filters: filters.into(),
            properties,
        }
    }

    /// Applies all the filters from the collection to the given map.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the filters from the collection can't be
    /// applied to the map.
    pub fn apply(&self, source: &Map<T>, destination: &mut Map<T>) -> Result<(), TisuError>
    where
        T: Clone + PartialEq,
    {
        for filter in &self.filters {
            filter.apply(source, destination)?;
        }

        Ok(())
    }

    pub fn push(&mut self, filter: Filter<T>) {
        self.filters.push(filter);
    }
}
