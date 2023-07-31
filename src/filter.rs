use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use regex::Regex;

use crate::{
    io::read_xml_file,
    model::{Coordinate, SquaredFilter},
    parser::{compile_coordinate_regex, extract_first_coordinate_from_text},
    utils::{is_point_in_area, is_point_more_than_x_distance_from_filter},
};

type SafeSharedVec = Arc<Mutex<Vec<PathBuf>>>;

/// Filters all the tracks that at a distance longer than the provided distance from the provided
/// area. Filtering is based on the first point found in the file using a regex on latitude and
/// longitude
///
/// # Panics
///
/// Panics if:
/// 1. There is a problem with the threads.
/// 1. The directory does not exist.
pub fn prefilter_files(
    paths: Vec<PathBuf>,
    area: &SquaredFilter,
    distance: f32,
    threads: usize,
) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let thread_pool = build_thread_pool(threads);

    let nearby_paths = SafeSharedVec::default();
    let area_paths = SafeSharedVec::default();

    let re = compile_coordinate_regex();

    thread_pool.install(|| {
        paths.into_par_iter().for_each(|path| {
            let nearby_paths_clone = Arc::clone(&nearby_paths);
            let area_paths_clone = Arc::clone(&area_paths);
            let re_clone = re.clone();

            let coordinate = extract_first_coordinate_from_file(&path, re_clone);
            match coordinate {
                Some(coordinate) if is_point_in_area(area, &coordinate) => {
                    let mut area_paths = area_paths_clone.lock().unwrap();
                    area_paths.push(path);
                }
                Some(coordinate)
                    if !is_point_more_than_x_distance_from_filter(area, &coordinate, distance) =>
                {
                    let mut nearby_paths = nearby_paths_clone.lock().unwrap();
                    nearby_paths.push(path);
                }
                _ => (),
            }
        });
    });

    let area = Arc::try_unwrap(area_paths).unwrap().into_inner().unwrap();
    let nearby = Arc::try_unwrap(nearby_paths).unwrap().into_inner().unwrap();
    (area, nearby)
}

/// .
/// Filters all the tracks that do not have at least one point in the provided area by looping
/// through all the points
///
/// # Panics
///
/// Panics if:
/// 1. There is a problem with the threads.
/// 1. The directory does not exist.
pub fn filter_tracks_outside_area(
    paths: Vec<PathBuf>,
    area: &SquaredFilter,
    threads: usize,
) -> Vec<PathBuf> {
    let thread_pool = build_thread_pool(threads);

    let filtered_paths = SafeSharedVec::default();
    thread_pool.install(|| {
        paths.into_par_iter().for_each(|path| {
            let filtered_paths_clone = Arc::clone(&filtered_paths);
            let area_clone = area.clone();

            if file_contains_point_in_area(&path, &area_clone) {
                let mut filtered_paths = filtered_paths_clone.lock().unwrap();
                filtered_paths.push(path.clone());
            }
        });
    });

    Arc::try_unwrap(filtered_paths)
        .unwrap()
        .into_inner()
        .unwrap()
}

/// Return a thread pool with the given number of threads
fn build_thread_pool(threads: usize) -> rayon::ThreadPool {
    let thread_pool = ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("The thread pool could not be created");
    thread_pool
}

/// Use a regex to find the first point the file
fn extract_first_coordinate_from_file(path: &PathBuf, re: Regex) -> Option<Coordinate> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        
        let line = match line {
            Ok(line) => line,
            Err(_) => continue, // Skip lines with errors
        };

        let coordinate = extract_first_coordinate_from_text(&re, line.as_str());
        match coordinate {
            Some(coordinate) => return Some(coordinate),
            None => continue,
        }
    }
    None
}

/// Check the files that are further away the given distance from the target area
fn file_contains_point_in_area(path: &PathBuf, area: &SquaredFilter) -> bool {
    let coordinates = read_xml_file(&path);

    for coordinate in coordinates {
        if is_point_in_area(area, &coordinate) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::model::Coordinate;

    use super::*;

    const TEST_FILE: &str = "test/files/9244476879.gpx";

    #[test]
    fn test_file_is_not_in_area() {
        let area = SquaredFilter::new(
            Coordinate::new(8.5763870, 47.3753910),
            Coordinate::new(10.985259, 49.48004),
        );
        let path = PathBuf::from(TEST_FILE);

        assert_eq!(false, file_contains_point_in_area(&path, &area));
    }

    #[test]
    fn test_file_is_in_area() {
        let area = SquaredFilter::new(
            Coordinate::new(-12.5763870, 165.3753910),
            Coordinate::new(-10.985259, 167.48004),
        );
        let path = PathBuf::from(TEST_FILE);

        assert_eq!(true, file_contains_point_in_area(&path, &area));
    }
}
