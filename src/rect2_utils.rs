use crate::{rect2::Rect2u, vector2::Vector2u};

pub fn get_rect_above(rect: Rect2u, y: usize) -> Option<Rect2u> {
    if y > 0 {
        let size = Vector2u::new(rect.get_size().x, y);
        Some(Rect2u::new(rect.get_position(), size).unwrap())
    } else {
        None
    }
}

pub fn get_rect_left(rect: Rect2u, x: usize) -> Option<Rect2u> {
    if x > 0 {
        let size = Vector2u::new(x, rect.get_size().y);
        Some(Rect2u::new(rect.get_position(), size).unwrap())
    } else {
        None
    }
}

pub fn get_rect_below(rect: Rect2u, y: usize) -> Option<Rect2u> {
    if y < rect.get_size().y - 1 {
        let position = Vector2u::new(rect.get_position().x, y + 1);
        let size = Vector2u::new(rect.get_size().x, rect.get_size().y - y - 1);
        Some(Rect2u::new(position, size).unwrap())
    } else {
        None
    }
}

pub fn get_rect_right(rect: Rect2u, x: usize) -> Option<Rect2u> {
    if x < rect.get_size().x - 1 {
        let position = Vector2u::new(x + 1, rect.get_position().y);
        let size = Vector2u::new(rect.get_size().x - x - 1, rect.get_size().y);
        Some(Rect2u::new(position, size).unwrap())
    } else {
        None
    }
}
