use std::path::{Path, PathBuf};

use clap::Parser;
use regen::regen_error::RegenError;
use regen::tiled_filter_loader::TiledFilterLoader;
use regen::tiled_map_loader::TiledMapLoader;
use regen::vector2::Vector2u;
use tiled::Loader;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CmdLineArgs {
    /// Output file path
    #[arg(short, long, default_value = "output.tmx")]
    output: PathBuf,
    /// Input file path
    #[arg(short, long)]
    input: PathBuf,
    /// Filters file path
    #[arg(short, long)]
    filters: PathBuf,
}

fn load_tile_size(file: impl AsRef<Path>) -> Result<Vector2u, RegenError> {
    let mut loader = Loader::new();
    let tsx_tileset = loader
        .load_tsx_tileset(file)
        .map_err(|_| RegenError::InvalidArgument)?;
    Ok((tsx_tileset.tile_width, tsx_tileset.tile_height).into())
}

fn main() {
    let args = CmdLineArgs::parse();

    let load_result = TiledMapLoader::load(&args.input).expect("Failed to load map");
    let filters = TiledFilterLoader::load(&args.filters, Some(4)).expect("Failed to load filters");
    let new_map = filters
        .apply(&load_result.map_layers[0].map)
        .expect("Failed to apply filters");
    let tile_size = load_tile_size(&load_result.tileset_path).expect("Failed to load tileset");
    TiledMapLoader::save(&args.output, &new_map, tile_size, &load_result.tileset_path)
        .expect("Failed to save map");
}
