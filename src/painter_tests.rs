use crate::painter;
use crate::regen_error::RegenError;
use crate::{map::Map, rect2::Rect2u, rect2_utils};

#[test]
fn test_h_split_rect_failure() {
    let rect = Rect2u::new((1, 1).into(), (4, 4).into()).unwrap();
    let invalid_rect = Rect2u::new((1, 1).into(), (1, 1).into()).unwrap();

    assert!(rect2_utils::h_split_rect(rect, 0).is_err());
    assert!(rect2_utils::h_split_rect(rect, 4).is_err());
    assert!(rect2_utils::h_split_rect(invalid_rect, 1).is_err());
}

#[test]
fn test_v_split_rect_failure() {
    let rect = Rect2u::new((1, 1).into(), (4, 4).into()).unwrap();
    let invalid_rect = Rect2u::new((1, 1).into(), (1, 1).into()).unwrap();

    assert!(rect2_utils::v_split_rect(rect, 0).is_err());
    assert!(rect2_utils::v_split_rect(rect, 4).is_err());
    assert!(rect2_utils::v_split_rect(invalid_rect, 1).is_err());
}

#[test]
fn test_fill() {
    let mut map = Map::<i32>::new((10, 10).into());

    painter::fill(&mut map, 42);

    for x in 0..10 {
        for y in 0..10 {
            assert_eq!(map.get((x, y).into()).unwrap(), &42);
        }
    }
}

#[test]
fn test_border_rect_success() {
    let mut map = Map::<i32>::new((10, 10).into());

    let result = painter::border_rect(&mut map, &(0, 0, 3, 3).try_into().unwrap(), 42);
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

    let result = painter::fill_rect(&mut map, &(0, 0, 3, 3).try_into().unwrap(), 42);

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

    let result = painter::fill_rect(&mut map, &(8, 8, 3, 3).try_into().unwrap(), 42);

    assert_eq!(result.err().unwrap(), RegenError::OutOfBounds);
}

#[test]
fn test_h_line_success() {
    let mut map = Map::<i32>::new((10, 10).into());

    let result = painter::h_line(&mut map, 1, 42);
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

    let result = painter::h_line(&mut map, 10, 42);

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

    let result = painter::h_line_rect(&mut map, &(1, 1, 3, 3).try_into().unwrap(), 1, 42);
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

    let result1 = painter::h_line_rect(&mut map, &(1, 1, 10, 10).try_into().unwrap(), 1, 42);
    let result2 = painter::h_line_rect(&mut map, &(1, 1, 3, 3).try_into().unwrap(), 3, 42);

    assert_eq!(result1.err().unwrap(), RegenError::OutOfBounds);
    assert_eq!(result2.err().unwrap(), RegenError::OutOfBounds);
}

#[test]
fn test_v_line_success() {
    let mut map = Map::<i32>::new((10, 10).into());

    let result = painter::v_line(&mut map, 1, 42);
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

    let result = painter::v_line(&mut map, 10, 42);

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

    let result = painter::v_line_rect(&mut map, &(1, 1, 3, 3).try_into().unwrap(), 1, 42);
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

    let result1 = painter::v_line_rect(&mut map, &(1, 1, 10, 10).try_into().unwrap(), 1, 42);
    let result2 = painter::v_line_rect(&mut map, &(1, 1, 3, 3).try_into().unwrap(), 3, 42);

    assert_eq!(result1.err().unwrap(), RegenError::OutOfBounds);
    assert_eq!(result2.err().unwrap(), RegenError::OutOfBounds);
}
