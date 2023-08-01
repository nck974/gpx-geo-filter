use std::path::PathBuf;

use clap::Parser;

use crate::config::GpxGeoFilterConfig;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
#[command(next_line_help = true)]
struct Cli {
    #[arg(long)]
    pub first_lat: f32,
    #[arg(long)]
    pub first_lon: f32,
    #[arg(long)]
    pub second_lat: f32,
    #[arg(long)]
    pub second_lon: f32,

    #[arg(long, short = 'd', default_value_t = 300.0)]
    pub distance: f32,

    #[arg(short = 'f', long)]
    pub folder: PathBuf,

    #[arg(short = 't', long, default_value_t = 8)]
    pub threads: usize,

    #[arg(short = 'c', long)]
    pub copy_to: Option<PathBuf>,
}

/// Parse the configuration that was provided in the command line and build a configuration
pub fn get_cli_arguments() -> GpxGeoFilterConfig {
    let cli = Cli::parse();

    let first_lat: f32 = cli.first_lat;
    let first_lon: f32 = cli.first_lon;
    let second_lat: f32 = cli.second_lat;
    let second_lon: f32 = cli.second_lon;

    let folder: PathBuf = cli.folder;
    let distance: f32 = cli.distance;
    let threads: usize = cli.threads;
    let copy_to: Option<PathBuf> = cli.copy_to;

    println!("> -----------------------------");
    println!("> first_lat: {:?}", first_lat);
    println!("> first_lon: {:?}", first_lon);
    println!("> second_lat: {:?}", second_lat);
    println!("> second_lon: {:?}", second_lon);

    println!("> distance: {:?}", distance);
    println!("> folder: {:?}", folder);
    println!("> threads: {:?}", threads);
    println!("> -----------------------------\n\n");

    GpxGeoFilterConfig {
        first_lat,
        first_lon,
        second_lat,
        second_lon,
        folder,
        distance,
        threads,
        copy_to,
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
