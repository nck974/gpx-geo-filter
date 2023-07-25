use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::{Arc, Mutex}, time::Instant,
};

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use regex::Regex;

use crate::{
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
    let re = compile_coordinate_regex();
    thread_pool.install(|| {
        paths.into_par_iter().for_each(|path| {
            let filtered_paths_clone = Arc::clone(&filtered_paths);
            let area_clone = area.clone();
            let re_clone = re.clone();

            let now = Instant::now();

            if file_contains_point_in_area(&path, re_clone, &area_clone) {
                let mut filtered_paths = filtered_paths_clone.lock().unwrap();
                filtered_paths.push(path.clone());
            }

            let elapsed = now.elapsed();
            println!("Elapsed: {:.2?} {path:?}", elapsed);
        
        });
    });

    Arc::try_unwrap(filtered_paths)
        .unwrap()
        .into_inner()
        .unwrap()
}

/// .Deprecated
pub fn prefilter_files_single_thread(
    paths: Vec<PathBuf>,
    area: &SquaredFilter,
    distance: f32,
) -> Vec<PathBuf> {
    let mut filtered_paths = Vec::new();
    let re = compile_coordinate_regex();
    for path in paths {
        if is_file_far_away_from_area(&path, re.clone(), area, distance) {
            continue;
        }
        filtered_paths.push(path);
    }
    filtered_paths
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
fn file_contains_point_in_area(path: &PathBuf, re: Regex, area: &SquaredFilter) -> bool {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_content = line.unwrap();
        
        // Skip regex if already line does not match
        if ! line_content.clone().contains("   <trkpt"){
            continue;
        }

        let coordinate = extract_first_coordinate_from_text(re.clone(), line_content.as_str());
        match coordinate {
            Some(coordinate) => {
                if is_point_in_area(area, &coordinate) {
                    return true;
                }
            }
            None => continue,
        }
    }
    false
}
