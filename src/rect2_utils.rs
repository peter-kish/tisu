use crate::rect2::Rect2u;
use crate::regen_error::RegenError;
use crate::vector2::{Vector2, Vector2u};

/// Splits the given rectangle into two horizontally at the given height and
/// returns the two newly created rectangles.
///
/// # Errors
///
/// Returns an error if the rectangle cannot be split at the given height.
pub fn h_split_rect(rect: Rect2u, height: u32) -> Result<(Rect2u, Rect2u), RegenError> {
    if height == 0 || height >= rect.size().y {
        return Err(RegenError::InvalidArgument);
    }
    if rect.size().y < 2 {
        return Err(RegenError::InvalidArgument);
    }

    let upper_rect_pos = rect.position();
    let upper_rect_size = Vector2::new(rect.size().x, height);
    let lower_rect_pos = Vector2::new(rect.position().x, rect.position().y + height);
    let lower_rect_size = Vector2::new(rect.size().x, rect.size().y - height);
    Ok((
        Rect2u::new(upper_rect_pos, upper_rect_size).unwrap(),
        Rect2u::new(lower_rect_pos, lower_rect_size).unwrap(),
    ))
}

/// Splits the given rectangle into two vertically at the given width and
/// returns the two newly created rectangles.
///
/// # Errors
///
/// Returns an error if the rectangle cannot be split at the given width.
pub fn v_split_rect(rect: Rect2u, width: u32) -> Result<(Rect2u, Rect2u), RegenError> {
    if width == 0 || width >= rect.size().x {
        return Err(RegenError::InvalidArgument);
    }
    if rect.size().x < 2 {
        return Err(RegenError::InvalidArgument);
    }

    let left_rect_pos = rect.position();
    let left_rect_size = Vector2::new(width, rect.size().y);
    let right_rect_pos = Vector2::new(rect.position().x + width, rect.position().y);
    let right_rect_size = Vector2::new(rect.size().x - width, rect.size().y);
    Ok((
        Rect2u::new(left_rect_pos, left_rect_size).unwrap(),
        Rect2u::new(right_rect_pos, right_rect_size).unwrap(),
    ))
}

/// Returns a sub-rectangle of the given rectangle that is above the given y
/// coordinate.
///
/// # Errors
///
/// Returns an error if the given y coordinate is out of rectangle bounds.
pub fn get_rect_above(rect: &Rect2u, y: u32) -> Result<Option<Rect2u>, RegenError> {
    if y > rect.size().y - 1 {
        Err(RegenError::OutOfBounds)
    } else if y > 0 {
        let size = Vector2u::new(rect.size().x, y);
        Ok(Some(Rect2u::new(rect.position(), size).unwrap()))
    } else {
        Ok(None)
    }
}

/// Returns a sub-rectangle of the given rectangle that is left of the given x
/// coordinate.
///
/// # Errors
///
/// Returns an error if the given x coordinate is out of rectangle bounds.
pub fn get_rect_left(rect: &Rect2u, x: u32) -> Result<Option<Rect2u>, RegenError> {
    if x > rect.size().x - 1 {
        Err(RegenError::OutOfBounds)
    } else if x > 0 {
        let size = Vector2u::new(x, rect.size().y);
        Ok(Some(Rect2u::new(rect.position(), size).unwrap()))
    } else {
        Ok(None)
    }
}

/// Returns a sub-rectangle of the given rectangle that is below the given y
/// coordinate.
///
/// # Errors
///
/// Returns an error if the given y coordinate is out of rectangle bounds.
pub fn get_rect_below(rect: &Rect2u, y: u32) -> Result<Option<Rect2u>, RegenError> {
    match y {
        _ if y > rect.size().y - 1 => Err(RegenError::OutOfBounds),
        _ if y < rect.size().y - 1 => {
            let position = Vector2u::new(rect.position().x, rect.position().y + y + 1);
            let size = Vector2u::new(rect.size().x, rect.size().y - y - 1);
            Ok(Some(Rect2u::new(position, size).unwrap()))
        }
        _ => Ok(None),
    }
}

/// Returns a sub-rectangle of the given rectangle that is right of the given x
/// coordinate.
///
/// # Errors
///
/// Returns an error if the given x coordinate is out of rectangle bounds.
pub fn get_rect_right(rect: &Rect2u, x: u32) -> Result<Option<Rect2u>, RegenError> {
    match x {
        _ if x > rect.size().x - 1 => Err(RegenError::OutOfBounds),
        _ if x < rect.size().x - 1 => {
            let position = Vector2u::new(rect.position().x + x + 1, rect.position().y);
            let size = Vector2u::new(rect.size().x - x - 1, rect.size().y);
            Ok(Some(Rect2u::new(position, size).unwrap()))
        }
        _ => Ok(None),
    }
}
