use crate::rect2::Rect2u;
use crate::regen_error::RegenError;
use crate::vector2::Vector2u;
use std::fmt::Display;

/// A generic map
#[derive(PartialEq, Debug, Clone)]
pub struct Map<T> {
    /// Size of the map
    size: Vector2u,
    /// Vector containing map data
    data: Vec<T>,
}

impl<T> Map<T> {
    /// Creates a map of a given size.
    pub fn new(size: Vector2u) -> Self
    where
        T: Clone + Default,
    {
        Self {
            size,
            data: vec![T::default(); (size.x * size.y).try_into().unwrap()],
        }
    }

    /// Creates a map of size M x N using the given map data.
    ///
    /// # Errors
    ///
    /// Returns an error if the given data buffer is empty.
    pub fn from_data<const M: usize, const N: usize>(data: [[T; M]; N]) -> Result<Self, RegenError>
    where
        T: Clone,
    {
        if data.is_empty() {
            Err(RegenError::InvalidArgument)
        } else {
            let map_width = data[0].len();
            let map_height = data.len();
            let mut total_data = vec![];
            for d in data {
                total_data.extend_from_slice(&d);
            }
            Ok(Self {
                size: Vector2u::new(
                    map_width.try_into().map_err(|_| RegenError::Unexpected)?,
                    map_height.try_into().map_err(|_| RegenError::Unexpected)?,
                ),
                data: total_data,
            })
        }
    }

    /// Returns the size of the map.
    pub fn size(&self) -> Vector2u {
        self.size
    }

    /// Returns a reference to the map data.
    pub fn data(&self) -> &[T] {
        self.data.as_slice()
    }

    /// Returns a mutable reference to the map data.
    pub fn mut_data(&mut self) -> &mut [T] {
        self.data.as_mut_slice()
    }

    /// Returns the value of the field at the given position.
    ///
    /// # Errors
    ///
    /// Returns an error if the given position is out of map bounds.
    pub fn get(&self, point: Vector2u) -> Result<&T, RegenError> {
        if point.x >= self.size().x || point.y >= self.size().y {
            Err(RegenError::OutOfBounds)
        } else {
            let idx = self.idx(point);
            self.data.get(idx).ok_or(RegenError::OutOfBounds)
        }
    }

    /// Sets the field at the given position to the given value.
    ///
    /// # Errors
    ///
    /// Returns an error if the given position is out of map bounds.
    pub fn set(&mut self, point: Vector2u, value: T) -> Result<(), RegenError> {
        if point.x >= self.size().x || point.y >= self.size().y {
            Err(RegenError::OutOfBounds)
        } else {
            let idx = self.idx(point);
            *(self.data.get_mut(idx).ok_or(RegenError::OutOfBounds)?) = value;
            Ok(())
        }
    }

    fn idx(&self, point: Vector2u) -> usize {
        (point.y * self.size.x + point.x).try_into().unwrap()
    }

    /// Maps the field values to type G using the given mapper.
    pub fn map<G, F>(&self, mapper: F) -> Map<G>
    where
        G: Clone + Default,
        F: Fn(&T) -> G,
    {
        Map {
            size: self.size,
            data: self.data.iter().map(mapper).collect(),
        }
    }

    /// Extracts a rectangular segment of the map as a separate map.
    ///
    /// # Errors
    ///
    /// Returns an error if the given rectangle exceeds map bounds.
    pub fn extract_segment(&self, segment_rect: Rect2u) -> Result<Map<T>, RegenError>
    where
        T: Clone + Default,
    {
        let map_rect = Rect2u::from(self);
        if map_rect.contains_rect(&segment_rect) {
            let mut segment = Map::new(segment_rect.size());
            for x in 0..segment.size().x {
                for y in 0..segment.size().y {
                    let value = self.get(segment_rect.position() + (x, y).into())?;
                    segment.set((x, y).into(), value.clone())?;
                }
            }
            Ok(segment)
        } else {
            Err(RegenError::InvalidArgument)
        }
    }

    /// Prints the map to stdout.
    pub fn print(&self)
    where
        T: Display,
    {
        for y in 0..self.size.x {
            for x in 0..self.size.y {
                let idx = self.idx((x, y).into());
                print!("{}", self.data.get(idx).unwrap());
            }
            println!();
        }
    }
}

impl<T> From<&Map<T>> for Rect2u {
    fn from(map: &Map<T>) -> Self {
        Rect2u::new(Vector2u::default(), map.size()).unwrap()
    }
}
