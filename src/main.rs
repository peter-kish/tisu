use map::Map;
use vector2::Vector2u;

mod map;
mod rect2;
mod rect2_utils;
mod regen_error;
mod vector2;

use clap::Parser;
use image::{Rgb, RgbImage};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CmdLineArgs {
    /// Output file path
    #[arg(short, long, default_value_t = String::from("output.png"))]
    output: String,
}

#[derive(Clone, Default)]
enum MapTile {
    #[default]
    Concrete,
    Asphalt,
    Grass,
    Hedge,
    Wall,
}

impl From<&MapTile> for Rgb<u8> {
    fn from(value: &MapTile) -> Self {
        match value {
            MapTile::Concrete => Rgb([128, 128, 128]),
            MapTile::Asphalt => Rgb([0, 0, 0]),
            MapTile::Grass => Rgb([0, 255, 0]),
            MapTile::Hedge => Rgb([0, 128, 0]),
            MapTile::Wall => Rgb([255, 128, 0]),
        }
    }
}

fn generate_map() -> Map<MapTile> {
    let mut map = Map::<MapTile>::new(Vector2u::new(20, 20));
    map.v_line(10, MapTile::Asphalt).expect("v_line_failed");
    map
}

fn draw_map(map: Map<MapTile>) -> RgbImage {
    let mut image = RgbImage::new(
        map.get_size().x.try_into().unwrap(),
        map.get_size().y.try_into().unwrap(),
    );
    for x in 0..map.get_size().x {
        for y in 0..map.get_size().y {
            image.put_pixel(
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                map.get((x, y).into()).unwrap().into(),
            );
        }
    }
    image
}

fn main() {
    let args = CmdLineArgs::parse();
    let map = generate_map();
    draw_map(map)
        .save(args.output)
        .expect("Failed to save image");
}
