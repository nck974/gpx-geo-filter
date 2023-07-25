use std::time::Instant;

use gpx_geo_filter::{
    filter::prefilter_files,
    io::collect_data,
    model::{Coordinate, SquaredFilter},
};

static DIRECTORY: &str = "samples";

fn main() {
    let now = Instant::now();

    let files = collect_data(DIRECTORY);
    println!("Files found: {}", files.len());

    let area = SquaredFilter::new(
        Coordinate::new(47.3753910, 8.5763870),
        Coordinate::new(49.48004, 10.985259),
    );
    let distance = 10.0;
    let files = prefilter_files(files, &area, distance);
    println!("Filtered files found: {}", files.len());

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
