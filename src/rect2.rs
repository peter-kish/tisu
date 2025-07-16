use std::ops::{Add, Sub};

use crate::{regen_error::RegenError, vector2::Vector2};

/// A generic 2d rectangle
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect2<T> {
    /// The position of the rectangle
    position: Vector2<T>,
    /// The size of the rectangle
    size: Vector2<T>,
}

pub type Rect2u = Rect2<u32>;

impl<T> Rect2<T> {
    /// Creates a rectangle at the given position and with the given size.
    ///
    /// # Errors
    ///
    /// Returns an error if the given size contains negative coordinates.
    pub fn new(position: Vector2<T>, size: Vector2<T>) -> Result<Self, RegenError>
    where
        T: PartialOrd + From<u16>,
    {
        if size.x < 0.into() || size.y < 0.into() {
            return Err(RegenError::InvalidArgument);
        }
        Ok(Self { position, size })
    }

    /// Returns position of the rectangle.
    pub fn position(&self) -> Vector2<T>
    where
        T: Copy,
    {
        self.position
    }

    /// Returns size of the rectangle.
    pub fn size(&self) -> Vector2<T>
    where
        T: Copy,
    {
        self.size
    }

    /// Checks if the rectangle contains the given point.
    pub fn contains_point(&self, point: Vector2<T>) -> bool
    where
        T: PartialOrd + Copy + Add<Output = T>,
    {
        point.x >= self.position.x
            && point.y >= self.position.y
            && point.x < self.position.x + self.size.x
            && point.y < self.position.y + self.size.y
    }

    /// Checks if the rectangle contains the given rectangle.
    pub fn contains_rect(&self, rect: &Rect2<T>) -> bool
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
