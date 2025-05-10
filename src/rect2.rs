use std::ops::Add;

use crate::vector2::Vector2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect2<T> {
    pub position: Vector2<T>,
    pub size: Vector2<T>,
}

pub type Rect2u = Rect2<usize>;

impl<T> Rect2<T> {
    pub fn new(position: Vector2<T>, size: Vector2<T>) -> Self {
        Self { position, size }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let expected_position = Vector2 { x: 1, y: 2 };
        let expected_size = Vector2 { x: 3, y: 4 };

        let rect = Rect2::<i32>::new(expected_position, expected_size);

        assert_eq!(rect.position, expected_position);
        assert_eq!(rect.size, expected_size);
    }

    #[test]
    fn test_contains_point() {
        let rect = Rect2::<i32>::new(Vector2::new(0, 0), Vector2::new(10, 10));

        assert!(rect.contains_point(Vector2::new(0, 0)));
        assert!(rect.contains_point(Vector2::new(9, 9)));
        assert!(!rect.contains_point(Vector2::new(10, 10)));
    }
}
