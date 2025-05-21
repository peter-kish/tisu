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

    pub fn apply(&self, input: &Map<T>) -> Result<Map<T>, RegenError>
    where
        Map<T>: Clone,
        T: Clone + PartialEq,
    {
        if input.get_size().x < self.pattern.get_size().x
            || input.get_size().y < self.pattern.get_size().y
        {
            Err(RegenError::InvalidArgument)
        } else {
            let mut result = input.clone();
            for x in 0..=input.get_size().x - self.pattern.get_size().x {
                for y in 0..=input.get_size().y - self.pattern.get_size().y {
                    let point = Vector2u::new(x, y);
                    if self.pattern_matches(input, point)? {
                        self.substitute(&mut result, point)?;
                    }
                }
            }
            Ok(result)
        }
    }

    fn pattern_matches(&self, input: &Map<T>, position: Vector2u) -> Result<bool, RegenError>
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

    fn substitute(&self, input: &mut Map<T>, position: Vector2u) -> Result<(), RegenError>
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

    #[test]
    fn test_apply_success() {
        // 1 0 1
        // 1 1 1
        // 1 0 1
        let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
        // 1 0
        let pattern = Map::<u32>::from_data([[1, 0]]).unwrap();
        // 0 1
        let substitute = Map::<u32>::from_data([[0, 1]]).unwrap();
        let filter = Filter::new(pattern, substitute).unwrap();
        // 0 1 1
        // 1 1 1
        // 0 1 1
        let expected_data = [0, 1, 1, 1, 1, 1, 0, 1, 1];

        let result = filter.apply(&map);

        assert!(result.is_ok());
        let result_map = result.unwrap();
        assert_eq!(result_map.get_data(), &expected_data);
    }

    #[test]
    fn test_apply_failure() {
        // 1 0 1
        // 1 1 1
        // 1 0 1
        let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
        // 1 0 0 0
        let pattern = Map::<u32>::from_data([[1, 0, 0, 0]]).unwrap();
        // 0 1 1 1
        let substitute = Map::<u32>::from_data([[0, 1, 1, 1]]).unwrap();
        let filter = Filter::new(pattern, substitute).unwrap();

        let result = filter.apply(&map);

        assert_eq!(result.err().unwrap(), RegenError::InvalidArgument);
    }
}
