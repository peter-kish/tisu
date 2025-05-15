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
    let r = map
        .border_rect((0, 0, 10, 10).try_into().unwrap(), 'b')
        .expect("border_rect failed")
        .unwrap();
    map.h_line_rect(r, 2, 'c').expect("h_line_rect failed");
    let (r1, _) = map.v_line_rect(r, 2, 'd').expect("v_line_rect failed");
    map.fill_rect(r1.unwrap(), 'e').expect("fill_rect failed");

    map.print();
}
