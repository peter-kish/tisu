#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

pub type Vector2u = Vector2<usize>;

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let vector = Vector2::<i32>::new(10, 10);

        assert_eq!(vector.x, 10);
        assert_eq!(vector.y, 10);
    }
}
