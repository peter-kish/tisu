use std::{
    num::TryFromIntError,
    ops::{Add, Sub},
};

/// A generic 2d vector
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2<T> {
    /// X coordinate of the vector
    pub x: T,
    /// Y coordinate of the vector
    pub y: T,
}

pub type Vector2u = Vector2<u32>;
pub type Vector2i = Vector2<i32>;

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
    /// Creates a vector with the given coordinates.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Creates a unit vector.
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

impl TryFrom<Vector2u> for Vector2i {
    type Error = TryFromIntError;
    fn try_from(value: Vector2u) -> Result<Self, Self::Error> {
        Ok(Vector2i::new(
            i32::try_from(value.x)?,
            i32::try_from(value.y)?,
        ))
    }
}

impl TryFrom<Vector2i> for Vector2u {
    type Error = TryFromIntError;
    fn try_from(value: Vector2i) -> Result<Self, Self::Error> {
        Ok(Vector2u::new(
            u32::try_from(value.x)?,
            u32::try_from(value.y)?,
        ))
    }
}
