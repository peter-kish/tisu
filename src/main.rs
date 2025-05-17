use std::result;

use map::Map;
use rect2::Rect2u32;
use regen::regen_error::RegenError;
use vector2::Vector2u32;

mod map;
mod rect2;
mod rect2_utils;
mod regen_error;
mod vector2;

use clap::Parser;
use image::{Rgb, RgbImage};
use rand::Rng;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CmdLineArgs {
    /// Output file path
    #[arg(short, long, default_value_t = String::from("output.png"))]
    output: String,
}

#[derive(Clone, Copy, Default)]
enum MapTile {
    #[default]
    Concrete,
    Asphalt,
    Grass,
    Hedge,
    Wall,
}

struct RoadConfiguration {
    wh: u32,
    margin: u32,
}

impl RoadConfiguration {
    fn get_min_size(&self) -> u32 {
        self.wh + 2 * self.margin
    }
}

struct RoadSplit {
    horizontal: bool,
    configuration: RoadConfiguration,
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

fn v_split_rect_with_road(
    map: &mut Map<MapTile>,
    rect: Rect2u32,
    road_width: u32,
    margin: u32,
) -> Result<(Rect2u32, Rect2u32), RegenError> {
    if margin * 2 + road_width >= rect.get_size().x {
        return Err(RegenError::InvalidArgument);
    }
    let road_start = rand::rng().random_range(margin..=rect.get_size().x - road_width - margin);
    let (left_rect, right_rect) = map.v_split_rect(rect, road_start).unwrap();
    let (road_rect, right_rect) = map.v_split_rect(right_rect, road_width).unwrap();
    map.fill_rect(road_rect, MapTile::Asphalt).unwrap();
    Ok((left_rect, right_rect))
}

fn h_split_rect_with_road(
    map: &mut Map<MapTile>,
    rect: Rect2u32,
    road_height: u32,
    margin: u32,
) -> Result<(Rect2u32, Rect2u32), RegenError> {
    if margin * 2 + road_height >= rect.get_size().y {
        return Err(RegenError::InvalidArgument);
    }
    let road_start = rand::rng().random_range(margin..=rect.get_size().y - road_height - margin);
    let (left_rect, right_rect) = map.h_split_rect(rect, road_start).unwrap();
    let (road_rect, right_rect) = map.h_split_rect(right_rect, road_height).unwrap();
    map.fill_rect(road_rect, MapTile::Asphalt).unwrap();
    Ok((left_rect, right_rect))
}

fn split_rect_with_road(map: &mut Map<MapTile>, rect: Rect2u32) -> Option<(Rect2u32, Rect2u32)> {
    if let Some(split) = get_road_split(rect) {
        if split.horizontal {
            h_split_rect_with_road(
                map,
                rect,
                split.configuration.wh,
                split.configuration.margin,
            )
            .ok()
        } else {
            v_split_rect_with_road(
                map,
                rect,
                split.configuration.wh,
                split.configuration.margin,
            )
            .ok()
        }
    } else {
        None
    }
}

fn get_road_split(rect: Rect2u32) -> Option<RoadSplit> {
    let road_configurations = [
        RoadConfiguration { wh: 7, margin: 7 },
        RoadConfiguration { wh: 5, margin: 6 },
        RoadConfiguration { wh: 3, margin: 5 },
    ];
    let horizontal = rect.get_size().y > rect.get_size().x;
    let wh = if rect.get_size().y > rect.get_size().x {
        rect.get_size().y
    } else {
        rect.get_size().x
    };
    for configuration in road_configurations {
        if wh > configuration.get_min_size() {
            return Some(RoadSplit {
                horizontal,
                configuration,
            });
        }
    }
    None
}

fn generate_roads(map: &mut Map<MapTile>, rect: Rect2u32) {
    if let Some(rects) = split_rect_with_road(map, rect) {
        generate_roads(map, rects.0);
        generate_roads(map, rects.1);
    }
}

fn generate_map() -> Map<MapTile> {
    let mut map = Map::<MapTile>::new(Vector2u32::new(64, 64));
    let map_rect: Rect2u32 = (&map).into();
    generate_roads(&mut map, map_rect);

    map
}

fn draw_map(map: Map<MapTile>) -> RgbImage {
    let mut image = RgbImage::new(map.get_size().x, map.get_size().y);
    for x in 0..map.get_size().x {
        for y in 0..map.get_size().y {
            image.put_pixel(x, y, map.get((x, y).into()).unwrap().into());
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
