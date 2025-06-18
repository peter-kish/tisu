use crate::rect2::Rect2u;
use crate::regen_error::RegenError;
use crate::vector2::{Vector2, Vector2u};

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

#[cfg(test)]
mod tests {
    use crate::vector2::Vector2;

    use super::*;

    #[test]
    fn test_h_split_rect_success() {
        let rect = Rect2u::new((1, 1).into(), (4, 4).into()).unwrap();

        let result = h_split_rect(rect, 1);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, (1, 1, 4, 1).try_into().unwrap());
        assert_eq!(result.unwrap().1, (1, 2, 4, 3).try_into().unwrap());
    }

    #[test]
    fn test_v_split_rect_success() {
        let rect = Rect2u::new((1, 1).into(), (4, 4).into()).unwrap();

        let result = v_split_rect(rect, 1);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, (1, 1, 1, 4).try_into().unwrap());
        assert_eq!(result.unwrap().1, (2, 1, 3, 4).try_into().unwrap());
    }

    #[test]
    fn test_get_rect_above_success() {
        let rect = Rect2u::new(Vector2::new(3, 3), Vector2::new(3, 3)).unwrap();

        let above_result = get_rect_above(&rect, 1);
        let above_upper_limit_result = get_rect_above(&rect, 0);
        let above_lower_limit_result = get_rect_above(&rect, 2);

        assert!(above_result.is_ok());
        let above = above_result.unwrap();
        assert!(above.is_some());
        assert_eq!(above.unwrap(), (3, 3, 3, 1).try_into().unwrap());

        assert!(above_upper_limit_result.is_ok());
        let above_upper_limit = above_upper_limit_result.unwrap();
        assert!(above_upper_limit.is_none());

        assert!(above_lower_limit_result.is_ok());
        let above_lower_limit = above_lower_limit_result.unwrap();
        assert!(above_lower_limit.is_some());
        assert_eq!(above_lower_limit.unwrap(), (3, 3, 3, 2).try_into().unwrap());
    }

    #[test]
    fn test_get_rect_above_failure() {
        let rect = Rect2u::new(Vector2::new(3, 3), Vector2::new(3, 3)).unwrap();

        let above_invalid_result = get_rect_above(&rect, 3);
        assert!(above_invalid_result.is_err());
        let above_invalid_result = get_rect_above(&rect, 30);
        assert!(above_invalid_result.is_err());
    }

    #[test]
    fn test_get_rect_left_success() {
        let rect = Rect2u::new(Vector2::new(3, 3), Vector2::new(3, 3)).unwrap();

        let left_result = get_rect_left(&rect, 1);
        let left_of_left_limit_result = get_rect_left(&rect, 0);
        let left_of_right_limit_result = get_rect_left(&rect, 2);

        assert!(left_result.is_ok());
        let left = left_result.unwrap();
        assert!(left.is_some());
        assert_eq!(left.unwrap(), (3, 3, 1, 3).try_into().unwrap());

        assert!(left_of_left_limit_result.is_ok());
        let left_of_left_limit = left_of_left_limit_result.unwrap();
        assert!(left_of_left_limit.is_none());

        assert!(left_of_right_limit_result.is_ok());
        let left_of_right_limit = left_of_right_limit_result.unwrap();
        assert!(left_of_right_limit.is_some());
        assert_eq!(
            left_of_right_limit.unwrap(),
            (3, 3, 2, 3).try_into().unwrap()
        );
    }

    #[test]
    fn test_get_rect_left_failure() {
        let rect = Rect2u::new(Vector2::new(3, 3), Vector2::new(3, 3)).unwrap();

        let left_of_invalid_result = get_rect_left(&rect, 3);
        assert!(left_of_invalid_result.is_err());
        let left_of_invalid_result = get_rect_left(&rect, 30);
        assert!(left_of_invalid_result.is_err());
    }

    #[test]
    fn test_get_rect_below_success() {
        let rect = Rect2u::new(Vector2::new(3, 3), Vector2::new(3, 3)).unwrap();

        let below_result = get_rect_below(&rect, 1);
        let below_upper_limit_result = get_rect_below(&rect, 0);
        let below_lower_limit_result = get_rect_below(&rect, 2);

        assert!(below_result.is_ok());
        let below = below_result.unwrap();
        assert!(below.is_some());
        assert_eq!(below.unwrap(), (3, 5, 3, 1).try_into().unwrap());

        assert!(below_upper_limit_result.is_ok());
        let below_upper_limit = below_upper_limit_result.unwrap();
        assert!(below_upper_limit.is_some());
        assert_eq!(below_upper_limit.unwrap(), (3, 4, 3, 2).try_into().unwrap());

        assert!(below_lower_limit_result.is_ok());
        let below_lower_limit = below_lower_limit_result.unwrap();
        assert!(below_lower_limit.is_none());
    }

    #[test]
    fn test_get_rect_below_failure() {
        let rect = Rect2u::new(Vector2::new(3, 3), Vector2::new(3, 3)).unwrap();

        let below_invalid_result = get_rect_below(&rect, 3);
        assert!(below_invalid_result.is_err());
        let below_invalid_result = get_rect_below(&rect, 30);
        assert!(below_invalid_result.is_err());
    }

    #[test]
    fn test_get_rect_right_success() {
        let rect = Rect2u::new(Vector2::new(3, 3), Vector2::new(3, 3)).unwrap();

        let right_result = get_rect_right(&rect, 1);
        let right_of_upper_limit_result = get_rect_right(&rect, 0);
        let right_of_lower_limit_result = get_rect_right(&rect, 2);

        assert!(right_result.is_ok());
        let right = right_result.unwrap();
        assert!(right.is_some());
        assert_eq!(right.unwrap(), (5, 3, 1, 3).try_into().unwrap());

        assert!(right_of_upper_limit_result.is_ok());
        let right_of_upper_limit = right_of_upper_limit_result.unwrap();
        assert!(right_of_upper_limit.is_some());
        assert_eq!(
            right_of_upper_limit.unwrap(),
            (4, 3, 2, 3).try_into().unwrap()
        );

        assert!(right_of_lower_limit_result.is_ok());
        let right_of_lower_limit = right_of_lower_limit_result.unwrap();
        assert!(right_of_lower_limit.is_none());
    }

    #[test]
    fn test_get_rect_right_failure() {
        let rect = Rect2u::new(Vector2::new(3, 3), Vector2::new(3, 3)).unwrap();

        let right_of_invalid_result = get_rect_right(&rect, 3);
        assert!(right_of_invalid_result.is_err());
        let right_of_invalid_result = get_rect_right(&rect, 30);
        assert!(right_of_invalid_result.is_err());
    }
}
