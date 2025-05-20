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
            data: vec![T::default(); (size.x * size.y).try_into().unwrap()],
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
        (point.y * self.size.x + point.x).try_into().unwrap()
    }

    pub fn h_split_rect(&self, rect: Rect2u, height: u32) -> Result<(Rect2u, Rect2u), RegenError> {
        if height == 0 || height >= rect.get_size().y {
            return Err(RegenError::InvalidArgument);
        }
        if rect.get_size().y < 2 {
            return Err(RegenError::InvalidArgument);
        }

        let upper_rect_pos = rect.get_position();
        let upper_rect_size = Vector2::new(rect.get_size().x, height);
        let lower_rect_pos = Vector2::new(rect.get_position().x, rect.get_position().y + height);
        let lower_rect_size = Vector2::new(rect.get_size().x, rect.get_size().y - height);
        Ok((
            Rect2u::new(upper_rect_pos, upper_rect_size).unwrap(),
            Rect2u::new(lower_rect_pos, lower_rect_size).unwrap(),
        ))
    }

    pub fn h_split(&self, height: u32) -> Result<(Rect2u, Rect2u), RegenError> {
        self.h_split_rect((0, 0, self.size.x, self.size.y).try_into().unwrap(), height)
    }

    pub fn v_split_rect(&self, rect: Rect2u, width: u32) -> Result<(Rect2u, Rect2u), RegenError> {
        if width == 0 || width >= rect.get_size().x {
            return Err(RegenError::InvalidArgument);
        }
        if rect.get_size().x < 2 {
            return Err(RegenError::InvalidArgument);
        }

        let left_rect_pos = rect.get_position();
        let left_rect_size = Vector2::new(width, rect.get_size().y);
        let right_rect_pos = Vector2::new(rect.get_position().x + width, rect.get_position().y);
        let right_rect_size = Vector2::new(rect.get_size().x - width, rect.get_size().y);
        Ok((
            Rect2u::new(left_rect_pos, left_rect_size).unwrap(),
            Rect2u::new(right_rect_pos, right_rect_size).unwrap(),
        ))
    }

    pub fn v_split(&self, width: u32) -> Result<(Rect2u, Rect2u), RegenError> {
        self.v_split_rect((0, 0, self.size.x, self.size.y).try_into().unwrap(), width)
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
        T: Clone,
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
                self.set((x, y).into(), value.clone())?;
            }
        }
        Ok(())
    }

    pub fn border_rect(&mut self, rect: Rect2u, value: T) -> Result<Option<Rect2u>, RegenError>
    where
        T: Clone,
    {
        if !Rect2u::new(Vector2u::default(), self.size)?.contains_rect(rect) {
            return Err(RegenError::OutOfBounds);
        }

        let x_min = rect.get_position().x;
        let x_max = rect.get_position().x + rect.get_size().x;
        let y_min = rect.get_position().y;
        let y_max = rect.get_position().y + rect.get_size().y;

        self.h_line_unsafe(y_min, x_min, x_max, value.clone());
        self.h_line_unsafe(y_max - 1, x_min, x_max, value.clone());
        self.v_line_unsafe(x_min, y_min + 1, y_max - 1, value.clone());
        self.v_line_unsafe(x_max - 1, y_min + 1, y_max - 1, value.clone());

        let rect_size = Vector2u::new(x_max - x_min - 2, y_max - y_min - 2);
        if rect_size.x > 0 && rect_size.y > 0 {
            let rect_pos = Vector2u::new(x_min, y_min) + Vector2u::one();
            Ok(Some(Rect2u::new(rect_pos, rect_size)?))
        } else {
            Ok(None)
        }
    }

    pub fn h_line(
        &mut self,
        y: u32,
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
        y: u32,
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
            self.h_line_unsafe(rect.get_position().y + y, x_min, x_max, value);

            Ok((
                rect2_utils::get_rect_above(rect, y)?,
                rect2_utils::get_rect_below(rect, y)?,
            ))
        }
    }

    fn h_line_unsafe(&mut self, y: u32, x_min: u32, x_max: u32, value: T)
    where
        T: Clone,
    {
        for x in x_min..x_max {
            let _ = self.set((x, y).into(), value.clone());
        }
    }

    pub fn v_line(
        &mut self,
        x: u32,
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
        x: u32,
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
            self.v_line_unsafe(rect.get_position().x + x, y_min, y_max, value);

            Ok((
                rect2_utils::get_rect_left(rect, x)?,
                rect2_utils::get_rect_right(rect, x)?,
            ))
        }
    }

    fn v_line_unsafe(&mut self, x: u32, y_min: u32, y_max: u32, value: T)
    where
        T: Clone,
    {
        for y in y_min..y_max {
            let _ = self.set((x, y).into(), value.clone());
        }
    }

    fn map<G, F>(&self, mapper: F) -> Map<G>
    where
        G: Clone + Default,
        F: Fn(&T) -> G,
    {
        Map {
            size: self.size,
            data: self.data.iter().map(mapper).collect(),
        }
    }

    pub fn print(&self)
    where
        T: Display,
    {
        for y in 0..self.size.x {
            for x in 0..self.size.y {
                let idx = self.get_idx((x, y).into());
                print!("{}", self.data.get(idx).unwrap());
            }
            println!();
        }
    }
}

impl<T> From<&Map<T>> for Rect2u {
    fn from(map: &Map<T>) -> Self {
        Rect2u::new(Vector2u::default(), map.get_size()).unwrap()
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
    fn test_h_split_rect_success() {
        let map = Map::<i32>::new((10, 10).into());
        let rect = Rect2u::new((1, 1).into(), (4, 4).into()).unwrap();

        let result = map.h_split_rect(rect, 1);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, (1, 1, 4, 1).try_into().unwrap());
        assert_eq!(result.unwrap().1, (1, 2, 4, 3).try_into().unwrap());
    }

    #[test]
    fn test_h_split_rect_failure() {
        let map = Map::<i32>::new((10, 10).into());
        let rect = Rect2u::new((1, 1).into(), (4, 4).into()).unwrap();
        let invalid_rect = Rect2u::new((1, 1).into(), (1, 1).into()).unwrap();

        assert!(map.h_split_rect(rect, 0).is_err());
        assert!(map.h_split_rect(rect, 4).is_err());
        assert!(map.h_split_rect(invalid_rect, 1).is_err());
    }

    #[test]
    fn test_v_split_rect_success() {
        let map = Map::<i32>::new((10, 10).into());
        let rect = Rect2u::new((1, 1).into(), (4, 4).into()).unwrap();

        let result = map.v_split_rect(rect, 1);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, (1, 1, 1, 4).try_into().unwrap());
        assert_eq!(result.unwrap().1, (2, 1, 3, 4).try_into().unwrap());
    }

    #[test]
    fn test_v_split_rect_failure() {
        let map = Map::<i32>::new((10, 10).into());
        let rect = Rect2u::new((1, 1).into(), (4, 4).into()).unwrap();
        let invalid_rect = Rect2u::new((1, 1).into(), (1, 1).into()).unwrap();

        assert!(map.v_split_rect(rect, 0).is_err());
        assert!(map.v_split_rect(rect, 4).is_err());
        assert!(map.v_split_rect(invalid_rect, 1).is_err());
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
    fn test_border_rect_success() {
        let mut map = Map::<i32>::new((10, 10).into());

        let result = map.border_rect((0, 0, 3, 3).try_into().unwrap(), 42);
        let expected_rect = Some((1, 1, 1, 1).try_into().unwrap());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_rect);
        for x in 0..10 {
            for y in 0..10 {
                let actual = map.get((x, y).into()).unwrap();
                if (0..3).contains(&x) && (0..3).contains(&y) && !(x == 1 && y == 1) {
                    assert_eq!(actual, &42);
                } else {
                    assert_eq!(actual, &0);
                }
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
                if (1..4).contains(&x) && y == 2 {
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
                if (1..4).contains(&y) && x == 2 {
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

    #[test]
    fn test_map() {
        let mut map = Map::<i32>::new((2, 2).into());
        _ = map.set((0, 0).into(), 1);
        _ = map.set((0, 1).into(), 2);
        _ = map.set((1, 0).into(), 3);
        _ = map.set((1, 1).into(), 4);

        let map2 = map.map(|&x| x.to_string());

        assert_eq!(map2.get((0, 0).into()).unwrap(), &"1".to_string());
        assert_eq!(map2.get((0, 1).into()).unwrap(), &"2".to_string());
        assert_eq!(map2.get((1, 0).into()).unwrap(), &"3".to_string());
        assert_eq!(map2.get((1, 1).into()).unwrap(), &"4".to_string());
    }
}
