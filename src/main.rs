use std::time::Instant;

use gpx_geo_filter::{
    filter::{prefilter_files, filter_tracks_outside_area},
    io::collect_data,
    model::{Coordinate, SquaredFilter},
};

static DIRECTORY: &str = "samples";
// static DIRECTORY: &str = r"C:\Users\nck\Development\py-geo-locator\gps_data";

fn main() {
    let now = Instant::now();

    let area = SquaredFilter::new(
        Coordinate::new(47.3753910, 8.5763870),
        Coordinate::new(49.48004, 10.985259),
    );
    let distance = 10.0;
    let threads: usize = 4;

    let files = collect_data(DIRECTORY);
    println!("Files found: {}", files.len());

    let files = prefilter_files(files, &area, distance, threads);
    println!("Pre-Filtered files found: {}", files.len());

    let files = filter_tracks_outside_area(files, &area, threads);
    println!("Filtered files found: {}", files.len());

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
