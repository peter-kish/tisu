use crate::map::Map;
use crate::rect2::{Rect2, Rect2u};
use crate::rect2_utils;
use crate::regen_error::RegenError;
use crate::vector2::{Vector2, Vector2u};

pub fn h_split<T>(map: &Map<T>, height: u32) -> Result<(Rect2u, Rect2u), RegenError> {
    rect2_utils::h_split_rect(
        (0, 0, map.size().x, map.size().y).try_into().unwrap(),
        height,
    )
}

pub fn v_split<T>(map: &Map<T>, width: u32) -> Result<(Rect2u, Rect2u), RegenError> {
    rect2_utils::v_split_rect(
        (0, 0, map.size().x, map.size().y).try_into().unwrap(),
        width,
    )
}

pub fn fill<T>(map: &mut Map<T>, value: T)
where
    T: Clone,
{
    for d in map.mut_data() {
        *d = value.clone();
    }
}

pub fn fill_rect<T>(map: &mut Map<T>, rect: &Rect2u, value: T) -> Result<(), RegenError>
where
    T: Clone,
{
    if !Rect2u::new(Vector2u::default(), map.size())?.contains_rect(rect) {
        return Err(RegenError::OutOfBounds);
    }

    let x_min = rect.position().x;
    let x_max = rect.position().x + rect.size().x;
    let y_min = rect.position().y;
    let y_max = rect.position().y + rect.size().y;

    for x in x_min..x_max {
        for y in y_min..y_max {
            map.set((x, y).into(), value.clone())?;
        }
    }
    Ok(())
}

pub fn border_rect<T>(
    map: &mut Map<T>,
    rect: &Rect2u,
    value: T,
) -> Result<Option<Rect2u>, RegenError>
where
    T: Clone,
{
    if !Rect2u::new(Vector2u::default(), map.size())?.contains_rect(rect) {
        return Err(RegenError::OutOfBounds);
    }

    let x_min = rect.position().x;
    let x_max = rect.position().x + rect.size().x;
    let y_min = rect.position().y;
    let y_max = rect.position().y + rect.size().y;

    h_line_unsafe(map, y_min, x_min, x_max, value.clone());
    h_line_unsafe(map, y_max - 1, x_min, x_max, value.clone());
    v_line_unsafe(map, x_min, y_min + 1, y_max - 1, value.clone());
    v_line_unsafe(map, x_max - 1, y_min + 1, y_max - 1, value.clone());

    let rect_size = Vector2u::new(x_max - x_min - 2, y_max - y_min - 2);
    if rect_size.x > 0 && rect_size.y > 0 {
        let rect_pos = Vector2u::new(x_min, y_min) + Vector2u::one();
        Ok(Some(Rect2u::new(rect_pos, rect_size)?))
    } else {
        Ok(None)
    }
}

pub fn h_line<T>(
    map: &mut Map<T>,
    y: u32,
    value: T,
) -> Result<(Option<Rect2u>, Option<Rect2u>), RegenError>
where
    T: Clone,
{
    h_line_rect(map, &Rect2::new(Vector2::default(), map.size())?, y, value)
}

pub fn h_line_rect<T>(
    map: &mut Map<T>,
    rect: &Rect2u,
    y: u32,
    value: T,
) -> Result<(Option<Rect2u>, Option<Rect2u>), RegenError>
where
    T: Clone,
{
    let rect_out_of_bounds = !Rect2u::new(Vector2u::default(), map.size())?.contains_rect(rect);
    let y_out_of_bounds = y >= rect.size().y;

    if rect_out_of_bounds || y_out_of_bounds {
        Err(RegenError::OutOfBounds)
    } else {
        let x_min = rect.position().x;
        let x_max = rect.position().x + rect.size().x;
        h_line_unsafe(map, rect.position().y + y, x_min, x_max, value);

        Ok((
            rect2_utils::get_rect_above(rect, y)?,
            rect2_utils::get_rect_below(rect, y)?,
        ))
    }
}

fn h_line_unsafe<T>(map: &mut Map<T>, y: u32, x_min: u32, x_max: u32, value: T)
where
    T: Clone,
{
    for x in x_min..x_max {
        let _ = map.set((x, y).into(), value.clone());
    }
}

pub fn v_line<T>(
    map: &mut Map<T>,
    x: u32,
    value: T,
) -> Result<(Option<Rect2u>, Option<Rect2u>), RegenError>
where
    T: Clone,
{
    v_line_rect(map, &Rect2::new(Vector2::default(), map.size())?, x, value)
}

pub fn v_line_rect<T>(
    map: &mut Map<T>,
    rect: &Rect2u,
    x: u32,
    value: T,
) -> Result<(Option<Rect2u>, Option<Rect2u>), RegenError>
where
    T: Clone,
{
    let rect_out_of_bounds = !Rect2u::new(Vector2u::default(), map.size())?.contains_rect(rect);
    let x_out_of_bounds = x >= rect.size().x;

    if rect_out_of_bounds || x_out_of_bounds {
        Err(RegenError::OutOfBounds)
    } else {
        let y_min = rect.position().y;
        let y_max = rect.position().y + rect.size().y;
        v_line_unsafe(map, rect.position().x + x, y_min, y_max, value);

        Ok((
            rect2_utils::get_rect_left(rect, x)?,
            rect2_utils::get_rect_right(rect, x)?,
        ))
    }
}

fn v_line_unsafe<T>(map: &mut Map<T>, x: u32, y_min: u32, y_max: u32, value: T)
where
    T: Clone,
{
    for y in y_min..y_max {
        let _ = map.set((x, y).into(), value.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        fill(&mut map, 42);

        for x in 0..10 {
            for y in 0..10 {
                assert_eq!(map.get((x, y).into()).unwrap(), &42);
            }
        }
    }

    #[test]
    fn test_border_rect_success() {
        let mut map = Map::<i32>::new((10, 10).into());

        let result = border_rect(&mut map, &(0, 0, 3, 3).try_into().unwrap(), 42);
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

        let result = fill_rect(&mut map, &(0, 0, 3, 3).try_into().unwrap(), 42);

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

        let result = fill_rect(&mut map, &(8, 8, 3, 3).try_into().unwrap(), 42);

        assert_eq!(result.err().unwrap(), RegenError::OutOfBounds);
    }

    #[test]
    fn test_h_line_success() {
        let mut map = Map::<i32>::new((10, 10).into());

        let result = h_line(&mut map, 1, 42);
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

        let result = h_line(&mut map, 10, 42);

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

        let result = h_line_rect(&mut map, &(1, 1, 3, 3).try_into().unwrap(), 1, 42);
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

        let result1 = h_line_rect(&mut map, &(1, 1, 10, 10).try_into().unwrap(), 1, 42);
        let result2 = h_line_rect(&mut map, &(1, 1, 3, 3).try_into().unwrap(), 3, 42);

        assert_eq!(result1.err().unwrap(), RegenError::OutOfBounds);
        assert_eq!(result2.err().unwrap(), RegenError::OutOfBounds);
    }

    #[test]
    fn test_v_line_success() {
        let mut map = Map::<i32>::new((10, 10).into());

        let result = v_line(&mut map, 1, 42);
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

        let result = v_line(&mut map, 10, 42);

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

        let result = v_line_rect(&mut map, &(1, 1, 3, 3).try_into().unwrap(), 1, 42);
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

        let result1 = v_line_rect(&mut map, &(1, 1, 10, 10).try_into().unwrap(), 1, 42);
        let result2 = v_line_rect(&mut map, &(1, 1, 3, 3).try_into().unwrap(), 3, 42);

        assert_eq!(result1.err().unwrap(), RegenError::OutOfBounds);
        assert_eq!(result2.err().unwrap(), RegenError::OutOfBounds);
    }
}
