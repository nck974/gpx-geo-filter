use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::{Arc, Mutex},
    // time::Instant,
};

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use regex::Regex;

use crate::{
    io::read_xml_file,
    model::SquaredFilter,
    parser::{compile_coordinate_regex, extract_first_coordinate_from_text},
    utils::{is_point_in_area, is_point_more_than_x_distance_from_filter},
};

/// .
/// Filters all the tracks that at a distance longer than the provided distance from the provided
/// area. Filtering is based on the first element found
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
) -> Vec<PathBuf> {
    let thread_pool = ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap();

    let filtered_paths = Arc::new(Mutex::new(Vec::new()));
    let re = compile_coordinate_regex();
    thread_pool.install(|| {
        paths.into_par_iter().for_each(|path| {
            let filtered_paths_clone = Arc::clone(&filtered_paths);
            let area_clone = area.clone();
            let re_clone = re.clone();

            if !is_file_far_away_from_area(&path, re_clone, &area_clone, distance) {
                let mut filtered_paths = filtered_paths_clone.lock().unwrap();
                filtered_paths.push(path);
            }
        });
    });

    Arc::try_unwrap(filtered_paths)
        .unwrap()
        .into_inner()
        .unwrap()
}

/// .
/// Filters all the tracks that do not have at least one point in the provided area
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
    let thread_pool = ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap();

    let filtered_paths = Arc::new(Mutex::new(Vec::new()));
    thread_pool.install(|| {
        paths.into_par_iter().for_each(|path| {
            let filtered_paths_clone = Arc::clone(&filtered_paths);
            let area_clone = area.clone();

            // let now = Instant::now();

            if file_contains_point_in_area(&path, &area_clone) {
                let mut filtered_paths = filtered_paths_clone.lock().unwrap();
                filtered_paths.push(path.clone());
            }

            // let elapsed = now.elapsed();
            // println!("Elapsed: {:.2?} {path:?}", elapsed);
        });
    });

    Arc::try_unwrap(filtered_paths)
        .unwrap()
        .into_inner()
        .unwrap()
}

/// Check the files that are further away the given distance from the target area
fn is_file_far_away_from_area(
    path: &PathBuf,
    re: Regex,
    area: &SquaredFilter,
    distance: f32,
) -> bool {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let coordinate = extract_first_coordinate_from_text(re.clone(), line.unwrap().as_str());
        match coordinate {
            Some(coordinate) => {
                if is_point_in_area(&area, &coordinate) {
                    return false;
                }
                if is_point_more_than_x_distance_from_filter(area, &coordinate, distance) {
                    return true;
                }
                return false;
            }
            None => continue,
        }
    }
    true
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
