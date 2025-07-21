use crate::{rect2::Rect2, tisu_error::TisuError, vector2::Vector2};

#[test]
fn test_constructor_success() {
    let expected_position = Vector2::new(1, 2);
    let expected_size = Vector2::new(3, 4);

    let result = Rect2::<i32>::new(expected_position, expected_size);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().position(), expected_position);
    assert_eq!(result.unwrap().size(), expected_size);
}

#[test]
fn test_constructor_failure() {
    let position = Vector2::new(1, 2);
    let size = Vector2::new(-3, 4);

    let result = Rect2::<i32>::new(position, size);

    assert_eq!(result.err().unwrap(), TisuError::InvalidArgument);
}

#[test]
fn test_from() {
    let result = Rect2::<i32>::try_from((1, 2, 3, 4));

    assert!(result.is_ok());
    assert_eq!(result.unwrap().position(), (1, 2).into());
    assert_eq!(result.unwrap().size(), (3, 4).into());
}

#[test]
fn test_from_failure() {
    let result = Rect2::<i32>::try_from((1, 2, -3, -4));

    assert_eq!(result.err().unwrap(), TisuError::InvalidArgument);
}

#[test]
fn test_contains_point() {
    let rect = Rect2::<i32>::try_from((0, 0, 10, 10)).unwrap();

    assert!(rect.contains_point(Vector2::default()));
    assert!(rect.contains_point((9, 9).into()));
    assert!(!rect.contains_point((10, 10).into()));
}

#[test]
fn test_contains_rect() {
    let rect = Rect2::<i32>::try_from((1, 1, 10, 10)).unwrap();
    let top_left_corner = Rect2::<i32>::try_from((1, 1, 3, 3)).unwrap();
    let bottom_right_corner = Rect2::<i32>::try_from((8, 8, 3, 3)).unwrap();
    let invalid_top_left_corner = Rect2::<i32>::try_from((0, 0, 3, 3)).unwrap();
    let invalid_bottom_right_corner = Rect2::<i32>::try_from((8, 8, 4, 4)).unwrap();
    let enclosing = Rect2::<i32>::try_from((0, 0, 12, 12)).unwrap();

    assert!(rect.contains_rect(&top_left_corner));
    assert!(rect.contains_rect(&bottom_right_corner));
    assert!(rect.contains_rect(&rect));
    assert!(!rect.contains_rect(&invalid_top_left_corner));
    assert!(!rect.contains_rect(&invalid_bottom_right_corner));
    assert!(!rect.contains_rect(&enclosing));
}
