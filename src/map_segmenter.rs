use crate::map::Map;
use crate::rect2::Rect2u;
use crate::regen_error::RegenError;
use crate::vector2::Vector2u;

pub fn extract_segments<T>(map: &Map<T>, transparent_value: &T) -> Vec<Rect2u>
where
    T: PartialEq,
{
    let mut result = vec![];

    let mut it = Vector2u::default();
    while let Ok(rect_start) = find_rect_start(map, transparent_value, it) {
        it = rect_start;
        let rect_size = find_rect_size(map, transparent_value, it);
        if let Ok(rect) = Rect2u::new(it, rect_size) {
            result.push(rect);
        }
        it.x += rect_size.x;
    }

    result
}

fn is_field_transparent<T>(map: &Map<T>, transparent_value: &T, field: Vector2u) -> bool
where
    T: PartialEq,
{
    let map_rect = Rect2u::from(map);
    if !map_rect.contains_point(field) {
        true
    } else if let Ok(field_value) = map.get(field) {
        field_value == transparent_value
    } else {
        true
    }
}

fn is_rect_start<T>(map: &Map<T>, transparent_value: &T, field: Vector2u) -> bool
where
    T: PartialEq,
{
    !is_field_transparent(map, transparent_value, field)
        && is_field_transparent(map, transparent_value, field - (1, 0).into())
        && is_field_transparent(map, transparent_value, field - (0, 1).into())
}

fn find_rect_start<T>(
    map: &Map<T>,
    transparent_value: &T,
    from: Vector2u,
) -> Result<Vector2u, RegenError>
where
    T: PartialEq,
{
    let mut it = from;
    while it.y < map.get_size().y {
        while it.x < map.get_size().x {
            if is_rect_start(map, transparent_value, it) {
                return Ok(it);
            }
            it.x += 1;
        }
        it.y += 1;
        it.x = 0;
    }

    Err(RegenError::InvalidArgument)
}

fn find_rect_size<T>(map: &Map<T>, transparent_value: &T, field: Vector2u) -> Vector2u
where
    T: PartialEq,
{
    if is_field_transparent(map, transparent_value, field) {
        Vector2u::default()
    } else {
        let mut size = Vector2u::default();

        for x in field.x..map.get_size().x {
            if is_field_transparent(map, transparent_value, (x, field.y).into()) {
                size.x = x - field.x;
                break;
            }
        }
        for y in field.y..map.get_size().y {
            if is_field_transparent(map, transparent_value, (field.x, y).into()) {
                size.y = y - field.y;
                break;
            }
        }

        size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_field_transparent() {
        // 1 0
        // 0 0
        let map = Map::<u32>::from_data([[1, 0], [0, 0]]).unwrap();

        assert!(!is_field_transparent(&map, &0, (0, 0).into()));
        assert!(is_field_transparent(&map, &0, (1, 0).into()));
        assert!(is_field_transparent(&map, &0, (2, 0).into()));
    }

    #[test]
    fn test_is_rect_start() {
        // 0 0 0 0
        // 0 1 1 0
        // 0 0 0 0
        let map = Map::<u32>::from_data([[0, 0, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]]).unwrap();

        assert!(!is_rect_start(&map, &0, (0, 0).into()));
        assert!(!is_rect_start(&map, &0, (1, 0).into()));
        assert!(is_rect_start(&map, &0, (1, 1).into()));
        assert!(!is_rect_start(&map, &0, (2, 1).into()));
        assert!(!is_rect_start(&map, &0, (3, 1).into()));
        assert!(!is_rect_start(&map, &0, (4, 1).into()));
    }

    #[test]
    fn test_find_rect_start_success() {
        // 0 0 0 0
        // 0 1 1 0
        // 0 0 0 0
        let map = Map::<u32>::from_data([[0, 0, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]]).unwrap();

        let result = find_rect_start(&map, &0, (0, 0).into());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (1, 1).into());
    }

    #[test]
    fn test_find_rect_start_failure() {
        // 0 0 0 0
        // 0 0 0 0
        // 0 0 0 0
        let map = Map::<u32>::from_data([[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]).unwrap();

        let result = find_rect_start(&map, &0, (0, 0).into());

        assert_eq!(result.err().unwrap(), RegenError::InvalidArgument);
    }

    #[test]
    fn test_find_rect_size() {
        // 0 0 0 0
        // 0 1 1 0
        // 0 0 0 0
        let map = Map::<u32>::from_data([[0, 0, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]]).unwrap();

        assert_eq!(find_rect_size(&map, &0, (0, 0).into()), (0, 0).into());
        assert_eq!(find_rect_size(&map, &0, (1, 1).into()), (2, 1).into());
    }

    #[test]
    fn test_extract_segments() {
        // 0 0 0 0 0 0
        // 0 1 1 0 1 0
        // 0 0 0 0 0 0
        // 0 1 1 0 1 0
        // 0 1 1 0 0 0
        // 0 0 0 0 0 0
        let map = Map::<u32>::from_data([
            [0, 0, 0, 0, 0, 0],
            [0, 1, 1, 0, 1, 0],
            [0, 0, 0, 0, 0, 0],
            [0, 1, 1, 0, 1, 0],
            [0, 1, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
        ])
        .unwrap();
        let expected_rects = [
            Rect2u::try_from((1, 1, 2, 1)).unwrap(),
            Rect2u::try_from((4, 1, 1, 1)).unwrap(),
            Rect2u::try_from((1, 3, 2, 2)).unwrap(),
            Rect2u::try_from((4, 3, 1, 1)).unwrap(),
        ];

        let result = extract_segments(&map, &0);

        assert_eq!(result, expected_rects);
    }
}
