use crate::vector2::Vector2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect2<T> {
    pub position: Vector2<T>,
    pub size: Vector2<T>,
}

type Rect2u = Rect2<usize>;

impl<T> Rect2<T> {
    pub fn new(position: Vector2<T>, size: Vector2<T>) -> Self {
        Self { position, size }
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
}
