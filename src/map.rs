use crate::rect2::{Rect2, Rect2u};
use crate::rect2_utils;
use crate::regen_error::RegenError;
use crate::vector2::{Vector2, Vector2u};
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
        T: Clone,
    {
        for d in &mut self.data {
            *d = value.clone();
        }
    }

    pub fn fill_rect(&mut self, rect: Rect2u, value: T) -> Result<(), RegenError>
    where
        T: Copy,
    {
        if !Rect2u::new(Vector2u::default(), self.size)?.contains_rect(rect) {
            return Err(RegenError::OutOfBounds);
        }

        let x_min = rect.get_position().x;
        let x_max = rect.get_position().x + rect.get_size().x;
        let y_min = rect.get_position().y;
        let y_max = rect.get_position().y + rect.get_size().y;

        for x in x_min..x_max {
            for y in y_min..y_max {
                self.set((x, y).into(), value)?;
            }
        }
        Ok(())
    }

    pub fn h_line(
        &mut self,
        y: usize,
        value: T,
    ) -> Result<(Option<Rect2u>, Option<Rect2u>), RegenError>
    where
        T: Clone,
    {
        self.h_line_rect(Rect2::new(Vector2::default(), self.size)?, y, value)
    }

    pub fn h_line_rect(
        &mut self,
        rect: Rect2u,
        y: usize,
        value: T,
    ) -> Result<(Option<Rect2u>, Option<Rect2u>), RegenError>
    where
        T: Clone,
    {
        let rect_out_of_bounds = !Rect2u::new(Vector2u::default(), self.size)?.contains_rect(rect);
        let y_out_of_bounds = y >= rect.get_size().y;

        if rect_out_of_bounds || y_out_of_bounds {
            Err(RegenError::OutOfBounds)
        } else {
            let x_min = rect.get_position().x;
            let x_max = rect.get_position().x + rect.get_size().x;
            self.h_line_unsafe(y, x_min, x_max, value);

            Ok((
                rect2_utils::get_rect_above(rect, y)?,
                rect2_utils::get_rect_below(rect, y)?,
            ))
        }
    }

    fn h_line_unsafe(&mut self, y: usize, x_min: usize, x_max: usize, value: T)
    where
        T: Clone,
    {
        for x in x_min..x_max {
            let _ = self.set((x, y).into(), value.clone());
        }
    }

    pub fn v_line(
        &mut self,
        x: usize,
        value: T,
    ) -> Result<(Option<Rect2u>, Option<Rect2u>), RegenError>
    where
        T: Clone,
    {
        self.v_line_rect(Rect2::new(Vector2::default(), self.size)?, x, value)
    }

    pub fn v_line_rect(
        &mut self,
        rect: Rect2u,
        x: usize,
        value: T,
    ) -> Result<(Option<Rect2u>, Option<Rect2u>), RegenError>
    where
        T: Clone,
    {
        let rect_out_of_bounds = !Rect2u::new(Vector2u::default(), self.size)?.contains_rect(rect);
        let x_out_of_bounds = x >= rect.get_size().x;

        if rect_out_of_bounds || x_out_of_bounds {
            Err(RegenError::OutOfBounds)
        } else {
            let y_min = rect.get_position().y;
            let y_max = rect.get_position().y + rect.get_size().y;
            self.v_line_unsafe(x, y_min, y_max, value);

            Ok((
                rect2_utils::get_rect_left(rect, x)?,
                rect2_utils::get_rect_right(rect, x)?,
            ))
        }
    }

    fn v_line_unsafe(&mut self, x: usize, y_min: usize, y_max: usize, value: T)
    where
        T: Clone,
    {
        for y in y_min..y_max {
            let _ = self.set((x, y).into(), value.clone());
        }
    }

    pub fn print(&self)
    where
        T: Display,
    {
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let idx = self.get_idx((x, y).into());
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
        let map = Map::<i32>::new((10, 10).into());

        let value = map.get((3, 3).into());

        assert!(value.is_ok());
        assert_eq!(value.unwrap(), &0);
    }

    #[test]
    fn test_get_failure() {
        let map = Map::<i32>::new((10, 10).into());

        let value = map.get((10, 10).into());

        assert_eq!(value.err().unwrap(), RegenError::OutOfBounds);
    }

    #[test]
    fn test_set_success() {
        let mut map = Map::<i32>::new((10, 10).into());
        let point = Vector2u::new(3, 3);

        let result = map.set(point, 42);
        let value = map.get(point).unwrap();

        assert!(result.is_ok());
        assert_eq!(value, &42);
    }

    #[test]
    fn test_set_failure() {
        let mut map = Map::<i32>::new((10, 10).into());
        let point = Vector2u::new(10, 10);

        let result = map.set(point, 42);

        assert_eq!(result.err().unwrap(), RegenError::OutOfBounds);
    }

    #[test]
    fn test_fill() {
        let mut map = Map::<i32>::new((10, 10).into());

        map.fill(42);

        for x in 0..10 {
            for y in 0..10 {
                assert_eq!(map.get((x, y).into()).unwrap(), &42);
            }
        }
    }

    #[test]
    fn test_fill_rect_success() {
        let mut map = Map::<i32>::new((10, 10).into());

        let result = map.fill_rect((0, 0, 3, 3).try_into().unwrap(), 42);

        assert!(result.is_ok());
        for x in 0..10 {
            for y in 0..10 {
                if (0..3).contains(&x) && (0..3).contains(&y) {
                    assert_eq!(map.get((x, y).into()).unwrap(), &42);
                } else {
                    assert_eq!(map.get((x, y).into()).unwrap(), &0);
                }
            }
        }
    }

    #[test]
    fn test_fill_rect_failure() {
        let mut map = Map::<i32>::new((10, 10).into());

        let result = map.fill_rect((8, 8, 3, 3).try_into().unwrap(), 42);

        assert_eq!(result.err().unwrap(), RegenError::OutOfBounds);
    }

    #[test]
    fn test_h_line_success() {
        let mut map = Map::<i32>::new((10, 10).into());

        let result = map.h_line(1, 42);
        let expected_upper_rect = Some((0, 0, 10, 1).try_into().unwrap());
        let expected_lower_rect = Some((0, 2, 10, 8).try_into().unwrap());

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, expected_upper_rect);
        assert_eq!(result.unwrap().1, expected_lower_rect);
        for x in 0..10 {
            for y in 0..10 {
                if y == 1 {
                    assert_eq!(map.get((x, y).into()).unwrap(), &42);
                } else {
                    assert_eq!(map.get((x, y).into()).unwrap(), &0);
                }
            }
        }
    }

    #[test]
    fn test_h_line_failure() {
        let mut map = Map::<i32>::new((10, 10).into());

        let result = map.h_line(10, 42);

        assert_eq!(result.err().unwrap(), RegenError::OutOfBounds);
        for x in 0..10 {
            for y in 0..10 {
                assert_eq!(map.get((x, y).into()).unwrap(), &0);
            }
        }
    }

    #[test]
    fn test_h_line_rect_success() {
        let mut map = Map::<i32>::new((10, 10).into());

        let result = map.h_line_rect((1, 1, 3, 3).try_into().unwrap(), 1, 42);
        let expected_upper_rect = Some((1, 1, 3, 1).try_into().unwrap());
        let expected_lower_rect = Some((1, 3, 3, 1).try_into().unwrap());

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, expected_upper_rect);
        assert_eq!(result.unwrap().1, expected_lower_rect);
        for x in 0..10 {
            for y in 0..10 {
                if (1..4).contains(&x) && y == 1 {
                    assert_eq!(map.get((x, y).into()).unwrap(), &42);
                } else {
                    assert_eq!(map.get((x, y).into()).unwrap(), &0);
                }
            }
        }
    }

    #[test]
    fn test_h_line_rect_failure() {
        let mut map = Map::<i32>::new((10, 10).into());

        let result1 = map.h_line_rect((1, 1, 10, 10).try_into().unwrap(), 1, 42);
        let result2 = map.h_line_rect((1, 1, 3, 3).try_into().unwrap(), 3, 42);

        assert_eq!(result1.err().unwrap(), RegenError::OutOfBounds);
        assert_eq!(result2.err().unwrap(), RegenError::OutOfBounds);
    }

    #[test]
    fn test_v_line_success() {
        let mut map = Map::<i32>::new((10, 10).into());

        let result = map.v_line(1, 42);
        let expected_left_rect = Some((0, 0, 1, 10).try_into().unwrap());
        let expected_right_rect = Some((2, 0, 8, 10).try_into().unwrap());

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, expected_left_rect);
        assert_eq!(result.unwrap().1, expected_right_rect);
        for x in 0..10 {
            for y in 0..10 {
                if x == 1 {
                    assert_eq!(map.get((x, y).into()).unwrap(), &42);
                } else {
                    assert_eq!(map.get((x, y).into()).unwrap(), &0);
                }
            }
        }
    }

    #[test]
    fn test_v_line_failure() {
        let mut map = Map::<i32>::new((10, 10).into());

        let result = map.v_line(10, 42);

        assert_eq!(result.err().unwrap(), RegenError::OutOfBounds);
        for x in 0..10 {
            for y in 0..10 {
                assert_eq!(map.get((x, y).into()).unwrap(), &0);
            }
        }
    }

    #[test]
    fn test_v_line_rect_success() {
        let mut map = Map::<i32>::new((10, 10).into());

        let result = map.v_line_rect((1, 1, 3, 3).try_into().unwrap(), 1, 42);
        let expected_left_rect = Some((1, 1, 1, 3).try_into().unwrap());
        let expected_right_rect = Some((3, 1, 1, 3).try_into().unwrap());

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, expected_left_rect);
        assert_eq!(result.unwrap().1, expected_right_rect);
        for x in 0..10 {
            for y in 0..10 {
                if (1..4).contains(&y) && x == 1 {
                    assert_eq!(map.get((x, y).into()).unwrap(), &42);
                } else {
                    assert_eq!(map.get((x, y).into()).unwrap(), &0);
                }
            }
        }
    }

    #[test]
    fn test_v_line_rect_failure() {
        let mut map = Map::<i32>::new((10, 10).into());

        let result1 = map.v_line_rect((1, 1, 10, 10).try_into().unwrap(), 1, 42);
        let result2 = map.v_line_rect((1, 1, 3, 3).try_into().unwrap(), 3, 42);

        assert_eq!(result1.err().unwrap(), RegenError::OutOfBounds);
        assert_eq!(result2.err().unwrap(), RegenError::OutOfBounds);
    }
}
