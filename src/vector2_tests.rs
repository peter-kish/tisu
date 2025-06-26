use crate::vector2::Vector2;

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
