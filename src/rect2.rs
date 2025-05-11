use std::ops::{Add, Sub};

use crate::{regen_error::RegenError, vector2::Vector2};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect2<T> {
    position: Vector2<T>,
    size: Vector2<T>,
}

pub type Rect2u = Rect2<usize>;

impl<T> Rect2<T> {
    pub fn new(position: Vector2<T>, size: Vector2<T>) -> Result<Self, RegenError>
    where
        T: PartialOrd + From<u16>,
    {
        if size.x < 0.into() || size.y < 0.into() {
            return Err(RegenError::InvalidArgument);
        }
        Ok(Self { position, size })
    }

    pub fn get_position(&self) -> Vector2<T>
    where
        T: Copy,
    {
        self.position
    }

    pub fn get_size(&self) -> Vector2<T>
    where
        T: Copy,
    {
        self.size
    }

    pub fn contains_point(&self, point: Vector2<T>) -> bool
    where
        T: PartialOrd + Copy + Add<Output = T>,
    {
        point.x >= self.position.x
            && point.y >= self.position.y
            && point.x < self.position.x + self.size.x
            && point.y < self.position.y + self.size.y
    }

    pub fn contains_rect(&self, rect: Rect2<T>) -> bool
    where
        T: PartialOrd + Copy + Add<Output = T> + Sub<Output = T> + From<u16>,
    {
        self.contains_point(rect.position)
            && self.contains_point(rect.position + rect.size - Vector2::one())
    }
}

impl<T> TryFrom<(T, T, T, T)> for Rect2<T>
where
    T: PartialOrd + std::convert::From<u16>,
{
    type Error = RegenError;

    fn try_from(value: (T, T, T, T)) -> Result<Self, Self::Error> {
        let position = Vector2::new(value.0, value.1);
        let size = Vector2::new(value.2, value.3);
        Self::new(position, size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor_success() {
        let expected_position = Vector2 { x: 1, y: 2 };
        let expected_size = Vector2 { x: 3, y: 4 };

        let result = Rect2::<i32>::new(expected_position, expected_size);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().get_position(), expected_position);
        assert_eq!(result.unwrap().get_size(), expected_size);
    }

    #[test]
    fn test_constructor_failure() {
        let position = Vector2 { x: 1, y: 2 };
        let size = Vector2 { x: -3, y: 4 };

        let result = Rect2::<i32>::new(position, size);

        assert!(result.is_err());
    }

    #[test]
    fn test_from() {
        let result = Rect2::<i32>::try_from((1, 2, 3, 4));

        assert!(result.is_ok());
        assert_eq!(result.unwrap().get_position(), Vector2 { x: 1, y: 2 });
        assert_eq!(result.unwrap().get_size(), Vector2 { x: 3, y: 4 });
    }

    #[test]
    fn test_from_failure() {
        let result = Rect2::<i32>::try_from((1, 2, -3, -4));

        assert!(result.is_err());
    }

    #[test]
    fn test_contains_point() {
        let rect = Rect2::<i32>::try_from((0, 0, 10, 10)).expect("Rect2::new failed!");

        assert!(rect.contains_point(Vector2::default()));
        assert!(rect.contains_point(Vector2::new(9, 9)));
        assert!(!rect.contains_point(Vector2::new(10, 10)));
    }

    #[test]
    fn test_contains_rect() {
        let rect = Rect2::<i32>::try_from((1, 1, 10, 10)).expect("Rect2::new failed!");
        let top_left_corner = Rect2::<i32>::try_from((1, 1, 3, 3)).expect("Rect2::new failed!");
        let bottom_right_corner = Rect2::<i32>::try_from((8, 8, 3, 3)).expect("Rect2::new failed!");
        let invalid_top_left_corner =
            Rect2::<i32>::try_from((0, 0, 3, 3)).expect("Rect2::new failed!");
        let invalid_bottom_right_corner =
            Rect2::<i32>::try_from((8, 8, 4, 4)).expect("Rect2::new failed!");
        let enclosing = Rect2::<i32>::try_from((0, 0, 12, 12)).expect("Rect2::new failed!");

        assert!(rect.contains_rect(top_left_corner));
        assert!(rect.contains_rect(bottom_right_corner));
        assert!(rect.contains_rect(rect));
        assert!(!rect.contains_rect(invalid_top_left_corner));
        assert!(!rect.contains_rect(invalid_bottom_right_corner));
        assert!(!rect.contains_rect(enclosing));
    }
}
