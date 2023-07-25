use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::{Arc, Mutex},
};

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

use crate::{
    model::SquaredFilter, parser::extract_first_coordinate_from_text,
    utils::is_point_more_than_x_distance_from_filter,
};

pub fn prefilter_files(paths: Vec<PathBuf>, area: &SquaredFilter, distance: f32) -> Vec<PathBuf> {
    let thread_pool = ThreadPoolBuilder::new().num_threads(64).build().unwrap();

    let filtered_paths = Arc::new(Mutex::new(Vec::new()));

    thread_pool.install(|| {
        paths.into_par_iter().for_each(|path| {
            let filtered_paths_clone = Arc::clone(&filtered_paths);
            let area_clone = area.clone();

            if !is_file_far_away_from_area(&path, &area_clone, distance) {
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

pub fn prefilter_files_single_thread(
    paths: Vec<PathBuf>,
    area: &SquaredFilter,
    distance: f32,
) -> Vec<PathBuf> {
    let mut filtered_paths = Vec::new();
    for path in paths {
        if is_file_far_away_from_area(&path, area, distance) {
            continue;
        }
        filtered_paths.push(path);
    }
    filtered_paths
}

/// Remove files that are far away from the target area to spend less time parsing files
fn is_file_far_away_from_area(path: &PathBuf, area: &SquaredFilter, distance: f32) -> bool {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let coordinate = extract_first_coordinate_from_text(line.unwrap().as_str());
        match coordinate {
            Some(coordinate) => {
                // println!("Some distance: {}, {} distance to {}", coordinate.longitude, coordinate.latitude, distance);
                // println!("Area is {}, {}, {}, {} ", area.top_left.longitude, area.top_right.latitude, area.bottom_right.longitude, area.bottom_right.latitude);
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
