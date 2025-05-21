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

    // TODO: Test
    pub fn pattern_matches(&self, input: &Map<T>, position: Vector2u) -> Result<bool, RegenError>
    where
        T: PartialEq,
    {
        for x in 0..self.pattern.get_size().x {
            for y in 0..self.pattern.get_size().y {
                let point = Vector2u::new(x, y);
                if input.get(position + point)? != self.pattern.get(point)? {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    // TODO: Test
    pub fn substitute(&self, input: &mut Map<T>, position: Vector2u) -> Result<(), RegenError>
    where
        T: Clone,
    {
        for x in 0..self.pattern.get_size().x {
            for y in 0..self.pattern.get_size().y {
                let point = Vector2u::new(x, y);
                input.set(position + point, self.substitute.get(point)?.clone())?;
            }
        }

        Ok(())
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
        let result = Filter::new(pattern.clone(), substitute.clone());

        assert_eq!(result.err().unwrap(), RegenError::InvalidArgument);
    }
}
