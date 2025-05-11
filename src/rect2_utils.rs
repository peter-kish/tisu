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
        let position = Vector2u::new(rect.get_position().x, rect.get_position().y + y + 1);
        let size = Vector2u::new(rect.get_size().x, rect.get_size().y - y - 1);
        Some(Rect2u::new(position, size).unwrap())
    } else {
        None
    }
}

pub fn get_rect_right(rect: Rect2u, x: usize) -> Option<Rect2u> {
    if x < rect.get_size().x - 1 {
        let position = Vector2u::new(rect.get_position().x + x + 1, rect.get_position().y);
        let size = Vector2u::new(rect.get_size().x - x - 1, rect.get_size().y);
        Some(Rect2u::new(position, size).unwrap())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::vector2::Vector2;

    use super::*;

    #[test]
    fn test_get_rect_above() {
        let rect = Rect2u::new(Vector2::new(3, 3), Vector2::new(3, 3)).unwrap();

        let above = get_rect_above(rect, 1);
        let above_upper_limit = get_rect_above(rect, 0);
        let above_lower_limit = get_rect_above(rect, 2);

        assert!(above.is_some());
        assert_eq!(above.unwrap(), (3, 3, 3, 1).try_into().unwrap());

        assert!(above_upper_limit.is_none());

        assert!(above_lower_limit.is_some());
        assert_eq!(above_lower_limit.unwrap(), (3, 3, 3, 2).try_into().unwrap());
    }

    #[test]
    fn test_get_rect_left() {
        let rect = Rect2u::new(Vector2::new(3, 3), Vector2::new(3, 3)).unwrap();

        let left = get_rect_left(rect, 1);
        let left_of_left_limit = get_rect_left(rect, 0);
        let left_of_right_limit = get_rect_left(rect, 2);

        assert!(left.is_some());
        assert_eq!(left.unwrap(), (3, 3, 1, 3).try_into().unwrap());

        assert!(left_of_left_limit.is_none());

        assert!(left_of_right_limit.is_some());
        assert_eq!(
            left_of_right_limit.unwrap(),
            (3, 3, 2, 3).try_into().unwrap()
        );
    }

    #[test]
    fn test_get_rect_below() {
        let rect = Rect2u::new(Vector2::new(3, 3), Vector2::new(3, 3)).unwrap();

        let below = get_rect_below(rect, 1);
        let below_upper_limit = get_rect_below(rect, 0);
        let below_lower_limit = get_rect_below(rect, 2);

        assert!(below.is_some());
        assert_eq!(below.unwrap(), (3, 5, 3, 1).try_into().unwrap());

        assert!(below_upper_limit.is_some());
        assert_eq!(below_upper_limit.unwrap(), (3, 4, 3, 2).try_into().unwrap());

        assert!(below_lower_limit.is_none());
    }

    #[test]
    fn test_get_rect_right() {
        let rect = Rect2u::new(Vector2::new(3, 3), Vector2::new(3, 3)).unwrap();

        let right = get_rect_right(rect, 1);
        let right_of_upper_limit = get_rect_right(rect, 0);
        let right_of_lower_limit = get_rect_right(rect, 2);

        assert!(right.is_some());
        assert_eq!(right.unwrap(), (5, 3, 1, 3).try_into().unwrap());

        assert!(right_of_upper_limit.is_some());
        assert_eq!(
            right_of_upper_limit.unwrap(),
            (4, 3, 2, 3).try_into().unwrap()
        );

        assert!(right_of_lower_limit.is_none());
    }
}
