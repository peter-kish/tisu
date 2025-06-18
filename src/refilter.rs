use clap::Parser;
use regen::filter;
use regen::tiled_map_converter::TiledMapConverter;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CmdLineArgs {
    /// Output file path
    #[arg(short, long, default_value_t = String::from("output.tmx"))]
    output: String,
    /// Input file path
    #[arg(short, long)]
    input: String,
    /// Filters file path
    #[arg(short, long)]
    filters: String,
}

fn main() {
    let args = CmdLineArgs::parse();

    let load_result = TiledMapConverter::load(&args.input).expect("Failed to load map");
    let filters =
        filter::load_tiled_filters(&args.filters, Some(4)).expect("Failed to load filters");
    let new_map = filters
        .apply(&load_result.map_layers[0].map)
        .expect("Failed to apply filters");
    TiledMapConverter::save(
        args.output.as_str(),
        &new_map,
        (16, 16).into(),
        "data/tileset.tsx",
    )
    .expect("Failed to save map");
}
