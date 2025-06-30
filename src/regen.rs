use std::path::PathBuf;

use regen::map::Map;
use regen::painter;
use regen::rect2::Rect2u;
use regen::rect2_utils;
use regen::regen_error::RegenError;
use regen::vector2::Vector2u;

use clap::Parser;
use image::{imageops, Rgb, RgbImage};
use rand::{random_range, Rng};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CmdLineArgs {
    /// Output file path
    #[arg(short, long)]
    output: PathBuf,
    /// Output zoom level
    #[arg(short, long, default_value_t = 1)]
    zoom: u32,
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
    width_or_height: u32,
    margin: u32,
    tile: MapTile,
}

impl RoadConfiguration {
    fn min_size(&self) -> u32 {
        self.width_or_height + 2 * self.margin
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
    if margin * 2 + road_width >= rect.size().x {
        return Err(RegenError::InvalidArgument);
    }
    let road_start = rand::rng().random_range(margin..=rect.size().x - road_width - margin);
    let (left_rect, right_rect) = rect2_utils::v_split_rect(rect, road_start).unwrap();
    let (road_rect, right_rect) = rect2_utils::v_split_rect(right_rect, road_width).unwrap();
    painter::fill_rect(map, &road_rect, road_tile).unwrap();
    Ok((left_rect, right_rect))
}

fn h_split_rect_with_road(
    map: &mut Map<MapTile>,
    rect: Rect2u,
    road_height: u32,
    margin: u32,
    road_tile: MapTile,
) -> Result<(Rect2u, Rect2u), RegenError> {
    if margin * 2 + road_height >= rect.size().y {
        return Err(RegenError::InvalidArgument);
    }
    let road_start = rand::rng().random_range(margin..=rect.size().y - road_height - margin);
    let (left_rect, right_rect) = rect2_utils::h_split_rect(rect, road_start).unwrap();
    let (road_rect, right_rect) = rect2_utils::h_split_rect(right_rect, road_height).unwrap();
    painter::fill_rect(map, &road_rect, road_tile).unwrap();
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
                split.configuration.width_or_height,
                split.configuration.margin,
                split.configuration.tile,
            )
            .ok()
        } else {
            v_split_rect_with_road(
                map,
                rect,
                split.configuration.width_or_height,
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
    let horizontal = rect.size().y > rect.size().x;
    let width_or_height = if rect.size().y > rect.size().x {
        rect.size().y
    } else {
        rect.size().x
    };
    for configuration in road_configurations {
        if width_or_height > configuration.min_size() {
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
            width_or_height: 7,
            margin: 17,
            tile: MapTile::Asphalt,
        },
        RoadConfiguration {
            width_or_height: 5,
            margin: 12,
            tile: MapTile::Asphalt,
        },
        RoadConfiguration {
            width_or_height: 3,
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
    if rect.size().x >= MIN_HEDGE_SIZE && rect.size().y >= MIN_HEDGE_SIZE {
        if let Ok(Some(park_inner)) = painter::border_rect(map, &rect, MapTile::Hedge) {
            painter::h_line_rect(map, &rect, rect.size().y / 2, MapTile::Grass).unwrap();
            painter::v_line_rect(map, &rect, rect.size().x / 2, MapTile::Grass).unwrap();
            _ = painter::fill_rect(map, &park_inner, MapTile::Grass);
        }
    } else {
        _ = painter::fill_rect(map, &rect, MapTile::Grass);
    }
}

fn generate_building(map: &mut Map<MapTile>, rect: Rect2u) {
    if let Ok(Some(building_inner)) = painter::border_rect(map, &rect, MapTile::Wall) {
        map.set(
            (
                rect.position().x + random_range(1..rect.size().x - 1),
                rect.position().y + rect.size().y - 1,
            )
                .into(),
            MapTile::Concrete,
        )
        .unwrap();

        _ = painter::fill_rect(map, &building_inner, MapTile::Concrete);
    }
}

fn generate_block(map: &mut Map<MapTile>, rect: Rect2u) {
    const MIN_BLOCK_SIZE: u32 = 5;
    if rect.size().x < MIN_BLOCK_SIZE || rect.size().y < MIN_BLOCK_SIZE {
        _ = painter::fill_rect(map, &rect, MapTile::Concrete);
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
        if let Ok(Some(block_inner)) = painter::border_rect(map, block, MapTile::Concrete) {
            if let Some(rects) = split_rect_with_road(
                map,
                block_inner,
                &[RoadConfiguration {
                    width_or_height: 1,
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

fn draw_map(map: Map<MapTile>, zoom: u32) -> RgbImage {
    let mut image = RgbImage::new(map.size().x, map.size().y);
    for x in 0..map.size().x {
        for y in 0..map.size().y {
            image.put_pixel(x, y, map.get((x, y).into()).unwrap().into());
        }
    }

    imageops::resize(
        &image,
        image.width() * zoom,
        image.height() * zoom,
        imageops::FilterType::Nearest,
    )
}

fn main() {
    let args = CmdLineArgs::parse();
    let map = generate_map();
    draw_map(map, args.zoom)
        .save(args.output)
        .expect("Failed to save image");
}
