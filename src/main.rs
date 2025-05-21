use map::Map;
use rect2::Rect2u;
use regen::regen_error::RegenError;
use vector2::Vector2u;

mod map;
mod rect2;
mod rect2_utils;
mod regen_error;
mod vector2;

use clap::Parser;
use image::{imageops, Rgb, RgbImage};
use rand::{random_range, Rng};

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

#[derive(Clone, Copy)]
struct RoadConfiguration {
    wh: u32,
    margin: u32,
    tile: MapTile,
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
    rect: Rect2u,
    road_width: u32,
    margin: u32,
    road_tile: MapTile,
) -> Result<(Rect2u, Rect2u), RegenError> {
    if margin * 2 + road_width >= rect.get_size().x {
        return Err(RegenError::InvalidArgument);
    }
    let road_start = rand::rng().random_range(margin..=rect.get_size().x - road_width - margin);
    let (left_rect, right_rect) = map.v_split_rect(rect, road_start).unwrap();
    let (road_rect, right_rect) = map.v_split_rect(right_rect, road_width).unwrap();
    map.fill_rect(&road_rect, road_tile).unwrap();
    Ok((left_rect, right_rect))
}

fn h_split_rect_with_road(
    map: &mut Map<MapTile>,
    rect: Rect2u,
    road_height: u32,
    margin: u32,
    road_tile: MapTile,
) -> Result<(Rect2u, Rect2u), RegenError> {
    if margin * 2 + road_height >= rect.get_size().y {
        return Err(RegenError::InvalidArgument);
    }
    let road_start = rand::rng().random_range(margin..=rect.get_size().y - road_height - margin);
    let (left_rect, right_rect) = map.h_split_rect(rect, road_start).unwrap();
    let (road_rect, right_rect) = map.h_split_rect(right_rect, road_height).unwrap();
    map.fill_rect(&road_rect, road_tile).unwrap();
    Ok((left_rect, right_rect))
}

fn split_rect_with_road(
    map: &mut Map<MapTile>,
    rect: Rect2u,
    road_configurations: &[RoadConfiguration],
) -> Option<(Rect2u, Rect2u)> {
    if let Some(split) = get_road_split(rect, road_configurations) {
        if split.horizontal {
            h_split_rect_with_road(
                map,
                rect,
                split.configuration.wh,
                split.configuration.margin,
                split.configuration.tile,
            )
            .ok()
        } else {
            v_split_rect_with_road(
                map,
                rect,
                split.configuration.wh,
                split.configuration.margin,
                split.configuration.tile,
            )
            .ok()
        }
    } else {
        None
    }
}

fn get_road_split(rect: Rect2u, road_configurations: &[RoadConfiguration]) -> Option<RoadSplit> {
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
                configuration: *configuration,
            });
        }
    }
    None
}

fn generate_roads(map: &mut Map<MapTile>, rect: Rect2u) -> Vec<Rect2u> {
    let road_configurations = vec![
        RoadConfiguration {
            wh: 7,
            margin: 17,
            tile: MapTile::Asphalt,
        },
        RoadConfiguration {
            wh: 5,
            margin: 12,
            tile: MapTile::Asphalt,
        },
        RoadConfiguration {
            wh: 3,
            margin: 8,
            tile: MapTile::Asphalt,
        },
    ];

    if let Some(rects) = split_rect_with_road(map, rect, &road_configurations) {
        let mut result1 = generate_roads(map, rects.0);
        let mut result2 = generate_roads(map, rects.1);
        result1.append(&mut result2);
        result1
    } else {
        vec![rect]
    }
}

fn generate_park(map: &mut Map<MapTile>, rect: Rect2u) {
    const MIN_HEDGE_SIZE: u32 = 7;
    if rect.get_size().x >= MIN_HEDGE_SIZE && rect.get_size().y >= MIN_HEDGE_SIZE {
        if let Ok(Some(park_inner)) = map.border_rect(&rect, MapTile::Hedge) {
            map.h_line_rect(&rect, rect.get_size().y / 2, MapTile::Grass)
                .unwrap();
            map.v_line_rect(&rect, rect.get_size().x / 2, MapTile::Grass)
                .unwrap();
            _ = map.fill_rect(&park_inner, MapTile::Grass);
        }
    } else {
        _ = map.fill_rect(&rect, MapTile::Grass);
    }
}

fn generate_building(map: &mut Map<MapTile>, rect: Rect2u) {
    if let Ok(Some(building_inner)) = map.border_rect(&rect, MapTile::Wall) {
        map.set(
            (
                rect.get_position().x + random_range(1..rect.get_size().x - 1),
                rect.get_position().y + rect.get_size().y - 1,
            )
                .into(),
            MapTile::Concrete,
        )
        .unwrap();

        _ = map.fill_rect(&building_inner, MapTile::Concrete);
    }
}

fn generate_block(map: &mut Map<MapTile>, rect: Rect2u) {
    const MIN_BLOCK_SIZE: u32 = 5;
    if rect.get_size().x < MIN_BLOCK_SIZE || rect.get_size().y < MIN_BLOCK_SIZE {
        _ = map.fill_rect(&rect, MapTile::Concrete);
        return;
    }

    let rand = random_range(0..10);
    if rand == 0 {
        generate_park(map, rect)
    } else {
        generate_building(map, rect)
    }
}

fn generate_blocks(map: &mut Map<MapTile>, blocks: &[Rect2u]) {
    for block in blocks {
        if let Ok(Some(block_inner)) = map.border_rect(block, MapTile::Concrete) {
            if let Some(rects) = split_rect_with_road(
                map,
                block_inner,
                &[RoadConfiguration {
                    wh: 1,
                    margin: 4,
                    tile: MapTile::Concrete,
                }],
            ) {
                generate_block(map, rects.0);
                generate_block(map, rects.1);
            } else {
                generate_block(map, block_inner);
            }
        }
    }
}

fn generate_map() -> Map<MapTile> {
    let mut map = Map::<MapTile>::new(Vector2u::new(64, 64));
    let map_rect: Rect2u = (&map).into();
    let blocks = generate_roads(&mut map, map_rect);
    generate_blocks(&mut map, &blocks);

    map
}

fn draw_map(map: Map<MapTile>) -> RgbImage {
    let mut image = RgbImage::new(map.get_size().x, map.get_size().y);
    for x in 0..map.get_size().x {
        for y in 0..map.get_size().y {
            image.put_pixel(x, y, map.get((x, y).into()).unwrap().into());
        }
    }

    const ZOOM: u32 = 4;
    imageops::resize(
        &image,
        image.width() * ZOOM,
        image.height() * ZOOM,
        imageops::FilterType::Nearest,
    )
}

fn main() {
    let args = CmdLineArgs::parse();
    let map = generate_map();
    draw_map(map)
        .save(args.output)
        .expect("Failed to save image");
}
