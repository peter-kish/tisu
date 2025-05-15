use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

pub type Vector2u32 = Vector2<u32>;

impl<T> Default for Vector2<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}

impl<T: Add<Output = T>> Add for Vector2<T> {
    type Output = Vector2<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Sub<Output = T>> Sub for Vector2<T> {
    type Output = Vector2<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn one() -> Self
    where
        T: From<u16>,
    {
        Vector2::new(T::from(1), T::from(1))
    }
}

impl<T> From<(T, T)> for Vector2<T>
where
    T: PartialOrd + std::convert::From<u16>,
{
    fn from(value: (T, T)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let vector = Vector2::<i32>::new(1, 2);

        assert_eq!(vector.x, 1);
        assert_eq!(vector.y, 2);
    }

    #[test]
    fn test_from() {
        let vector = Vector2::<i32>::from((1, 2));

        assert_eq!(vector.x, 1);
        assert_eq!(vector.y, 2);
    }

    #[test]
    fn test_add() {
        let vector1 = Vector2::<i32>::new(24, 42);
        let vector2 = Vector2::<i32>::new(42, 24);

        let result = vector1 + vector2;

        assert_eq!(result, Vector2::<i32>::new(66, 66));
    }

    #[test]
    fn test_sub() {
        let vector1 = Vector2::<i32>::new(24, 42);
        let vector2 = Vector2::<i32>::new(42, 24);

        let result = vector1 - vector2;

        assert_eq!(result, (-18, 18).into());
    }

    #[test]
    fn test_one() {
        let one_i32 = Vector2::<i32>::one();
        let one_u32 = Vector2::<u32>::one();
        let one_usize = Vector2::<u32>::one();
        let one_f32 = Vector2::<f32>::one();

        assert_eq!(one_i32.x, 1);
        assert_eq!(one_i32.y, 1);
        assert_eq!(one_u32.x, 1);
        assert_eq!(one_u32.y, 1);
        assert_eq!(one_usize.x, 1);
        assert_eq!(one_usize.y, 1);
        assert_eq!(one_f32.x, 1.0f32);
        assert_eq!(one_f32.y, 1.0f32);
    }
}
