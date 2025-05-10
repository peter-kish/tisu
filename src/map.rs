use crate::regen_error::RegenError;
use crate::vector2::Vector2u;
use std::fmt::Display;

pub struct Map<T> {
    size: Vector2u,
    data: Vec<T>,
}

impl<T> Map<T> {
    pub fn new(size: Vector2u) -> Self
    where
        T: Clone + Default,
    {
        Self {
            size,
            data: vec![T::default(); size.x * size.y],
        }
    }

    pub fn get_size(&self) -> Vector2u {
        self.size
    }

    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn get(&self, point: Vector2u) -> Result<&T, RegenError> {
        let idx = self.get_idx(point);
        self.data.get(idx).ok_or(RegenError::OutOfBounds)
    }

    pub fn set(&mut self, point: Vector2u, value: T) -> Result<(), RegenError> {
        let idx = self.get_idx(point);
        *(self.data.get_mut(idx).ok_or(RegenError::OutOfBounds)?) = value;
        Ok(())
    }

    fn get_idx(&self, point: Vector2u) -> usize {
        point.y * self.size.x + point.x
    }

    pub fn fill(&mut self, value: T)
    where
        T: Copy,
    {
        for d in &mut self.data {
            *d = value;
        }
    }

    pub fn print(&self)
    where
        T: Display,
    {
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let idx = self.get_idx(Vector2u::new(x, y));
                print!("{}", self.data.get(idx).unwrap());
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let expected_size = Vector2u::new(10, 10);
        let map = Map::<i32>::new(expected_size);

        let size = map.get_size();
        let data_len = map.get_data().len();

        assert_eq!(size, expected_size);
        assert_eq!(data_len, 100);
    }

    #[test]
    fn test_get_success() {
        let map = Map::<i32>::new(Vector2u::new(10, 10));

        let value = map.get(Vector2u::new(3, 3));

        assert!(value.is_ok());
        assert_eq!(value.unwrap(), &0);
    }

    #[test]
    fn test_get_failure() {
        let map = Map::<i32>::new(Vector2u::new(10, 10));

        let value = map.get(Vector2u::new(10, 10));

        assert!(value.is_err());
    }

    #[test]
    fn test_set_success() {
        let mut map = Map::<i32>::new(Vector2u::new(10, 10));
        let point = Vector2u::new(3, 3);

        let set_result = map.set(point, 42);
        let value = map.get(point).unwrap();

        assert!(set_result.is_ok());
        assert_eq!(value, &42);
    }

    #[test]
    fn test_set_failure() {
        let mut map = Map::<i32>::new(Vector2u::new(10, 10));
        let point = Vector2u::new(10, 10);

        let set_result = map.set(point, 42);

        assert!(set_result.is_err());
    }

    #[test]
    fn test_fill() {
        let mut map = Map::<i32>::new(Vector2u::new(10, 10));

        map.fill(42);

        for x in 0..10 {
            for y in 0..10 {
                assert_eq!(map.get(Vector2u::new(x, y)).unwrap(), &42);
            }
        }
    }
}
