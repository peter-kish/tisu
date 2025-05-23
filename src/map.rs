use crate::filter::Filter;
use crate::rect2::Rect2u;
use crate::regen_error::RegenError;
use crate::vector2::Vector2u;
use std::fmt::Display;

#[derive(PartialEq, Debug, Clone)]
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
                    map_width
                        .try_into()
                        .map_err(|_| RegenError::InvalidArgument)?,
                    map_height
                        .try_into()
                        .map_err(|_| RegenError::InvalidArgument)?,
                ),
                data: total_data,
            })
        }
    }

    pub fn get_size(&self) -> Vector2u {
        self.size
    }

    pub fn get_data(&self) -> &[T] {
        self.data.as_slice()
    }

    pub fn get_mut_data(&mut self) -> &mut [T] {
        self.data.as_mut_slice()
    }

    pub fn get(&self, point: Vector2u) -> Result<&T, RegenError> {
        if point.x >= self.get_size().x || point.y >= self.get_size().y {
            Err(RegenError::OutOfBounds)
        } else {
            let idx = self.get_idx(point);
            self.data.get(idx).ok_or(RegenError::OutOfBounds)
        }
    }

    pub fn set(&mut self, point: Vector2u, value: T) -> Result<(), RegenError> {
        if point.x >= self.get_size().x || point.y >= self.get_size().y {
            Err(RegenError::OutOfBounds)
        } else {
            let idx = self.get_idx(point);
            *(self.data.get_mut(idx).ok_or(RegenError::OutOfBounds)?) = value;
            Ok(())
        }
    }

    fn get_idx(&self, point: Vector2u) -> usize {
        (point.y * self.size.x + point.x).try_into().unwrap()
    }

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

    pub fn apply_filter(&self, filter: &Filter<T>) -> Result<Map<T>, RegenError>
    where
        Self: Clone,
        T: Clone + PartialEq,
    {
        if self.get_size().x < filter.get_pattern().get_size().x
            || self.get_size().y < filter.get_pattern().get_size().y
        {
            Err(RegenError::InvalidArgument)
        } else {
            let mut result = self.clone();
            for x in 0..=self.get_size().x - filter.get_pattern().get_size().x {
                for y in 0..=self.get_size().y - filter.get_pattern().get_size().y {
                    let point = Vector2u::new(x, y);
                    if filter.pattern_matches(self, point) {
                        filter.substitute(&mut result, point);
                    }
                }
            }
            Ok(result)
        }
    }

    pub fn apply_filters(&self, filters: &[Filter<T>]) -> Result<Map<T>, RegenError>
    where
        Self: Clone,
        T: Clone + PartialEq,
    {
        let mut maybe_result: Option<Map<T>> = None;
        for filter in filters {
            maybe_result = match maybe_result {
                Some(result) => Some(result.apply_filter(filter)?),
                None => Some(self.apply_filter(filter)?),
            };
        }
        maybe_result.ok_or(RegenError::InvalidArgument)
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
        for field in map.get_data() {
            assert_eq!(*field, 0);
        }
    }

    #[test]
    fn test_from_data_success() {
        let result = Map::<i32>::from_data([[1, 2], [3, 4]]);

        assert!(result.is_ok());
        let map = result.unwrap();
        assert_eq!(map.get_data(), [1, 2, 3, 4]);
        assert_eq!(map.get((0, 0).into()).unwrap(), &1);
        assert_eq!(map.get((1, 0).into()).unwrap(), &2);
        assert_eq!(map.get((0, 1).into()).unwrap(), &3);
        assert_eq!(map.get((1, 1).into()).unwrap(), &4);
    }

    #[test]
    fn test_from_data_failure() {
        let result = Map::<i32>::from_data::<0, 0>([]);

        assert_eq!(result.err().unwrap(), RegenError::InvalidArgument);
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

    #[test]
    fn test_apply_filter_success() {
        // 1 0 1
        // 1 1 1
        // 1 0 1
        let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
        // 1 0
        let pattern = Map::<u32>::from_data([[1, 0]]).unwrap();
        // 0 1
        let substitute = Map::<u32>::from_data([[0, 1]]).unwrap();
        let filter = Filter::new(pattern, substitute, 42).unwrap();
        // 0 1 1
        // 1 1 1
        // 0 1 1
        let expected_data = [0, 1, 1, 1, 1, 1, 0, 1, 1];

        let result = map.apply_filter(&filter);

        assert!(result.is_ok());
        let result_map = result.unwrap();
        assert_eq!(result_map.get_data(), &expected_data);
        assert_eq!(result_map.get_size(), map.get_size());
    }

    #[test]
    fn test_apply_filter_failure() {
        // 1 0 1
        // 1 1 1
        // 1 0 1
        let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
        // 1 0 0 0
        let pattern = Map::<u32>::from_data([[1, 0, 0, 0]]).unwrap();
        // 0 1 1 1
        let substitute = Map::<u32>::from_data([[0, 1, 1, 1]]).unwrap();
        let filter = Filter::new(pattern, substitute, 42).unwrap();

        let result = map.apply_filter(&filter);

        assert_eq!(result.err().unwrap(), RegenError::InvalidArgument);
    }

    #[test]
    fn test_apply_filter_patter_with_wildcard() {
        // 1 0 1
        // 1 1 1
        // 1 0 1
        let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
        // 1 W 1
        let pattern = Map::<u32>::from_data([[1, 2, 1]]).unwrap();
        // 0 1 0
        let substitute = Map::<u32>::from_data([[0, 1, 0]]).unwrap();
        let filter = Filter::new(pattern, substitute, 2).unwrap();
        // 0 1 0
        // 0 1 0
        // 0 1 0
        let expected_data = [0, 1, 0, 0, 1, 0, 0, 1, 0];

        let result = map.apply_filter(&filter);

        assert!(result.is_ok());
        let result_map = result.unwrap();
        assert_eq!(result_map.get_data(), &expected_data);
        assert_eq!(result_map.get_size(), map.get_size());
    }

    #[test]
    fn test_apply_filter_substitute_with_wildcard() {
        // 1 0 1
        // 1 1 1
        // 1 0 1
        let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
        // 1 0 1
        let pattern = Map::<u32>::from_data([[1, 0, 1]]).unwrap();
        // W 1 W
        let substitute = Map::<u32>::from_data([[2, 1, 2]]).unwrap();
        let filter = Filter::new(pattern, substitute, 2).unwrap();
        // 1 1 1
        // 1 1 1
        // 1 1 1
        let expected_data = [1, 1, 1, 1, 1, 1, 1, 1, 1];

        let result = map.apply_filter(&filter);

        assert!(result.is_ok());
        let result_map = result.unwrap();
        assert_eq!(result_map.get_data(), &expected_data);
        assert_eq!(result_map.get_size(), map.get_size());
    }

    #[test]
    fn test_apply_filters_success() {
        // 1 0 1
        // 1 1 1
        // 1 0 1
        let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
        // 1 0
        let pattern1 = Map::<u32>::from_data([[1, 0]]).unwrap();
        // 0 1
        let substitute1 = Map::<u32>::from_data([[0, 1]]).unwrap();
        let filter1 = Filter::new(pattern1, substitute1, 42).unwrap();
        // 0 1 1
        let pattern2 = Map::<u32>::from_data([[0, 1, 1]]).unwrap();
        // 0 0 0
        let substitute2 = Map::<u32>::from_data([[0, 0, 0]]).unwrap();
        let filter2 = Filter::new(pattern2, substitute2, 42).unwrap();
        // 0 0 0
        // 1 1 1
        // 0 0 0
        let expected_data = [0, 0, 0, 1, 1, 1, 0, 0, 0];

        let result = map.apply_filters(&[filter1, filter2]);

        assert!(result.is_ok());
        let result_map = result.unwrap();
        assert_eq!(result_map.get_data(), &expected_data);
        assert_eq!(result_map.get_size(), map.get_size());
    }

    #[test]
    fn test_apply_filters_failure() {
        // 1 0 1
        // 1 1 1
        // 1 0 1
        let map = Map::<u32>::from_data([[1, 0, 1], [1, 1, 1], [1, 0, 1]]).unwrap();
        let result = map.apply_filters(&[]);

        assert_eq!(result.err().unwrap(), RegenError::InvalidArgument);
    }
}
