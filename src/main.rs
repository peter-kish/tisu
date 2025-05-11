use map::Map;
use vector2::Vector2u;

mod map;
mod rect2;
mod rect2_utils;
mod regen_error;
mod vector2;

fn main() {
    let mut map = Map::<char>::new(Vector2u::new(10, 10));
    map.fill('a');
    map.h_line(1, 'b').expect("h_line failed");
    map.v_line(1, 'c').expect("v_line failed");
    map.fill_rect((8, 8, 2, 2).try_into().unwrap(), 'd')
        .expect("fill_rect failed");
    map.print();
}
