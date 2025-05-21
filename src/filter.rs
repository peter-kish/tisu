use crate::{map::Map, regen_error::RegenError, vector2::Vector2u};

pub struct Filter<T> {
    pattern: Map<T>,
    substitute: Map<T>,
}

impl<T> Filter<T> {
    pub fn new(pattern: Map<T>, substitute: Map<T>) -> Result<Self, RegenError> {
        if pattern.get_size() != substitute.get_size() {
            Err(RegenError::InvalidArgument)
        } else {
            Ok(Self {
                pattern,
                substitute,
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
        for x in 0..self.pattern.get_size().x {
            for y in 0..self.pattern.get_size().y {
                let point = Vector2u::new(x, y);
                if let Ok(input_field) = input.get(position + point) {
                    if let Ok(pattern_field) = self.pattern.get(point) {
                        if input_field != pattern_field {
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

    pub fn substitute(&self, input: &mut Map<T>, position: Vector2u)
    where
        T: Clone,
    {
        for x in 0..self.pattern.get_size().x {
            for y in 0..self.pattern.get_size().y {
                let point = Vector2u::new(x, y);
                if let Ok(substitute_field) = self.substitute.get(point) {
                    _ = input.set(position + point, substitute_field.clone());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor_success() {
        let pattern = Map::<u32>::new((2, 2).into());
        let substitute = Map::<u32>::new((2, 2).into());
        let result = Filter::new(pattern.clone(), substitute.clone());

        assert!(result.is_ok());
        let filter = result.unwrap();
        assert_eq!(filter.pattern, pattern);
        assert_eq!(filter.substitute, substitute);
    }

    #[test]
    fn test_constructor_failure() {
        let pattern = Map::<u32>::new((2, 2).into());
        let substitute = Map::<u32>::new((3, 2).into());
        let result = Filter::new(pattern, substitute);

        assert_eq!(result.err().unwrap(), RegenError::InvalidArgument);
    }

    #[test]
    fn test_pattern_matches() {
        let map = Map::<u32>::from_data([[1, 0], [0, 1]]).unwrap();
        let pattern = Map::<u32>::from_data([[1, 0]]).unwrap();
        let substitute = Map::<u32>::from_data([[1, 0]]).unwrap();
        let filter = Filter::new(pattern, substitute).unwrap();

        assert!(filter.pattern_matches(&map, (0, 0).into()));
        assert!(!filter.pattern_matches(&map, (0, 1).into()));
        assert!(!filter.pattern_matches(&map, (1, 1).into()));
    }

    #[test]
    fn test_substitute() {
        let mut map = Map::<u32>::from_data([[1, 0], [0, 1]]).unwrap();
        let pattern = Map::<u32>::from_data([[1, 0]]).unwrap();
        let substitute = Map::<u32>::from_data([[1, 0]]).unwrap();
        let filter = Filter::new(pattern, substitute).unwrap();

        filter.substitute(&mut map, (0, 1).into());
        assert_eq!(map.get_data(), [1, 0, 1, 0]);

        filter.substitute(&mut map, (1, 0).into());
        assert_eq!(map.get_data(), [1, 1, 1, 0]);
    }
}
