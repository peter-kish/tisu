use crate::{map::Map, regen_error::RegenError, vector2::Vector2u};

#[test]
fn test_constructor() {
    let expected_size = Vector2u::new(10, 10);
    let map = Map::<i32>::new(expected_size);

    let size = map.size();
    let data_len = map.data().len();

    assert_eq!(size, expected_size);
    assert_eq!(data_len, 100);
    for field in map.data() {
        assert_eq!(*field, 0);
    }
}

#[test]
fn test_from_data_success() {
    let result = Map::<i32>::from_data([[1, 2], [3, 4]]);

    assert!(result.is_ok());
    let map = result.unwrap();
    assert_eq!(map.data(), [1, 2, 3, 4]);
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
fn test_extract_segment_success() {
    // 0 1 0 1
    // 1 0 1 0
    // 0 1 0 1
    // 1 0 1 0
    let map =
        Map::<i32>::from_data([[0, 1, 0, 1], [1, 0, 1, 0], [0, 1, 0, 1], [1, 0, 1, 0]]).unwrap();

    let result = map.extract_segment((1, 1, 2, 2).try_into().unwrap());

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Map::from_data([[0, 1], [1, 0]]).unwrap());
}

#[test]
fn test_extract_segment_failure() {
    // 0 1 0 1
    // 1 0 1 0
    // 0 1 0 1
    // 1 0 1 0
    let map =
        Map::<i32>::from_data([[0, 1, 0, 1], [1, 0, 1, 0], [0, 1, 0, 1], [1, 0, 1, 0]]).unwrap();

    let result = map.extract_segment((3, 3, 2, 2).try_into().unwrap());

    assert_eq!(result.err().unwrap(), RegenError::InvalidArgument);
}
