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
