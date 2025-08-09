use std::path::{Path, PathBuf};

use clap::Parser;
use tiled::Loader;
use tisu::filter_importer::FilterImporter;
use tisu::map_exporter::MapExporter;
use tisu::map_importer::MapImporter;
use tisu::tiled_filter_importer::TiledFilterImporter;
use tisu::tiled_map_exporter::TiledMapExporter;
use tisu::tiled_map_importer::TiledMapImporter;
use tisu::tisu_error::TisuError;
use tisu::vector2::Vector2u;

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
    /// Wildcard tile index
    #[arg(short, long)]
    wildcard: Option<u32>,
}

fn load_tile_size(file: impl AsRef<Path>) -> Result<Vector2u, TisuError> {
    let mut loader = Loader::new();
    let tsx_tileset = loader
        .load_tsx_tileset(file)
        .map_err(|_| TisuError::InvalidArgument)?;
    Ok((tsx_tileset.tile_width, tsx_tileset.tile_height).into())
}

fn main() {
    let args = CmdLineArgs::parse();

    let load_result = TiledMapImporter::load(&args.input).expect("Failed to load map");
    let filters =
        TiledFilterImporter::load(&args.filters, args.wildcard).expect("Failed to load filters");
    let new_map = filters
        .apply(&load_result.map_layers[0].map)
        .expect("Failed to apply filters");
    let tile_size = load_tile_size(&load_result.tileset_path).expect("Failed to load tileset");
    TiledMapExporter::save(&args.output, &new_map, tile_size, &load_result.tileset_path)
        .expect("Failed to save map");
}
