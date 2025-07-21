use crate::map::Map;
use crate::rect2::Rect2u;
use crate::tisu_error::TisuError;
use crate::vector2::{Vector2i, Vector2u};

/// Returns a vector of rectangles in the given map separated by transparent
/// fields.
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

/// Checks if the field of the given map at the given coordinates is transparent.
pub fn is_field_transparent<T>(map: &Map<T>, transparent_value: &T, field: Vector2u) -> bool
where
    T: PartialEq,
{
    let field2i = match Vector2i::try_from(field) {
        Err(_) => return true, // Invalid (too large) value must be transparent
        Ok(x) => x,
    };
    is_field_transparent_impl(map, transparent_value, field2i)
}

fn is_field_transparent_impl<T>(map: &Map<T>, transparent_value: &T, field: Vector2i) -> bool
where
    T: PartialEq,
{
    if field.x < 0 || field.y < 0 {
        return true;
    }

    let map_rect = Rect2u::from(map);
    if !map_rect.contains_point((field.x as u32, field.y as u32).into()) {
        true
    } else if let Ok(field_value) = map.get((field.x as u32, field.y as u32).into()) {
        field_value == transparent_value
    } else {
        true
    }
}

/// Checks if the field of the given map at the given coordinates is the start
/// of a rectangle.
pub fn is_rect_start<T>(map: &Map<T>, transparent_value: &T, field: Vector2u) -> bool
where
    T: PartialEq,
{
    let field2i = match Vector2i::try_from(field) {
        Err(_) => return false, // Invalid (negative) value can't be rect start
        Ok(x) => x,
    };
    !is_field_transparent_impl(map, transparent_value, field2i)
        && is_field_transparent_impl(map, transparent_value, field2i - (1, 0).into())
        && is_field_transparent_impl(map, transparent_value, field2i - (0, 1).into())
}

/// Returns the starting coordinates of the first rectangle, starting from
/// top-left.
pub fn find_rect_start<T>(
    map: &Map<T>,
    transparent_value: &T,
    from: Vector2u,
) -> Result<Vector2u, TisuError>
where
    T: PartialEq,
{
    let mut it = from;
    while it.y < map.size().y {
        while it.x < map.size().x {
            if is_rect_start(map, transparent_value, it) {
                return Ok(it);
            }
            it.x += 1;
        }
        it.y += 1;
        it.x = 0;
    }

    Err(TisuError::NotFound)
}

/// Returns the size of the rectangle that starts at the given coordinates.
pub fn find_rect_size<T>(map: &Map<T>, transparent_value: &T, field: Vector2u) -> Vector2u
where
    T: PartialEq,
{
    if is_field_transparent(map, transparent_value, field) {
        Vector2u::default()
    } else {
        let mut size = Vector2u::default();

        for x in field.x..map.size().x {
            if is_field_transparent(map, transparent_value, (x, field.y).into()) {
                size.x = x - field.x;
                break;
            }
        }
        for y in field.y..map.size().y {
            if is_field_transparent(map, transparent_value, (field.x, y).into()) {
                size.y = y - field.y;
                break;
            }
        }

        size
    }
}
