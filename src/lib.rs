use std::{path::PathBuf, time::Instant};

use crate::{
    filter::{filter_tracks_outside_area, prefilter_files},
    io::{copy_gpx_file, read_files_in_folder},
    model::{Coordinate, SquaredFilter},
};

mod filter;
mod io;
mod model;
mod parser;
mod utils;

/// Filter the .gpx tracks found in the provided folder to only return the paths of the elements
/// that are within the area of the provided two points. The first one must be the most south-west.
///
/// Example:
///
/// ```rust
/// use gpx_geo_filter::filter_tracks;
///
/// static DIRECTORY: &str = "test/files";
///
/// let first_lat = 49.454470;
/// let first_lon = 10.954986;
/// let second_lat = 49.506443;
/// let second_lon = 11.030173;
///
/// let distance = 300.0;
/// let threads: usize = 12;
///
/// let files = filter_tracks(DIRECTORY, first_lat, first_lon, second_lat, second_lon, distance, threads);
/// ```
///
/// # Panics
///
/// Panics if:
/// 1. There is a problem with the threads.
/// 1. The directory does not exist.
pub fn filter_tracks(
    folder: &str,
    first_lat: f32,
    first_lon: f32,
    second_lat: f32,
    second_lon: f32,
    distance: f32,
    threads: usize,
) -> Vec<PathBuf> {
    let now = Instant::now();

    let area = SquaredFilter::new(
        Coordinate::new(first_lat, first_lon),
        Coordinate::new(second_lat, second_lon),
    );

    let files = read_files_in_folder(folder);
    println!("Files found: {}", files.len());

    let (mut files_area, files_nearby) = prefilter_files(files, &area, distance, threads);
    println!("Prefilter: files in area: {}", files_area.len());
    println!("Prefilter files close to the area: {}", files_nearby.len());

    let mut files = filter_tracks_outside_area(files_nearby, &area, threads);
    println!("Filtered files close to area: {}", files.len());

    files.append(&mut files_area);
    println!("Total files found: {}", files.len());

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    files
}

/// Copy the list of files in the provided directory with the same name
///
/// Example:
///
/// ```rust
/// use std::path::PathBuf;
/// use gpx_geo_filter::copy_gpx_files;
///
/// static OUTPUT: &str = r"output";
/// let files  = vec![PathBuf::from(r"test\files\8651674449.gpx")];
/// copy_gpx_files(files, OUTPUT);
/// ```
///
/// # Panics
///
/// Panics if:
/// 1. There directory does not exist.
/// 1. It is not possible to copy a file.
pub fn copy_gpx_files(files: Vec<PathBuf>, output: &str) {
    let now = Instant::now();

    for file in files {
        copy_gpx_file(output, file);
    }

    let elapsed = now.elapsed();
    println!("Elapsed copping: {:.2?}", elapsed);
}
