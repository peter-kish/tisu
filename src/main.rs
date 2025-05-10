use map::Map;
use vector2::Vector2u;

mod map;
mod regen_error;
mod vector2;

fn main() {
    let mut map = Map::<char>::new(Vector2u::new(10, 10));
    map.fill('a');
    map.print();
}
