mod map;
mod regen_error;
mod vector2;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{map::Map, vector2::Vector2u};

    #[test]
    fn it_works() {
        let map = Map::<char>::new(Vector2u::new(10, 10));
        assert_eq!(map.get_data().len(), 100);
    }
}
