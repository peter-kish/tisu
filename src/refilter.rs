use std::path::PathBuf;

use clap::Parser;
use regen::tiled_filter_loader::TiledFilterLoader;
use regen::tiled_map_loader::TiledMapLoader;

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

fn main() {
    let args = CmdLineArgs::parse();

    let load_result = TiledMapLoader::load(&args.input).expect("Failed to load map");
    let filters = TiledFilterLoader::load(&args.filters, Some(4)).expect("Failed to load filters");
    let new_map = filters
        .apply(&load_result.map_layers[0].map)
        .expect("Failed to apply filters");
    TiledMapLoader::save(
        &args.output,
        &new_map,
        (16, 16).into(),
        &load_result.tileset_path,
    )
    .expect("Failed to save map");
}
