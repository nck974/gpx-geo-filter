use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    model::SquaredFilter, parser::extract_first_coordinate_from_text,
    utils::is_point_more_than_x_distance_from_filter,
};

pub fn prefilter_files(paths: Vec<PathBuf>, area: &SquaredFilter, distance: f32) -> Vec<PathBuf> {
    let filtered_paths = Arc::new(Mutex::new(Vec::new()));
    let threads: Vec<_> = paths
        .into_iter()
        .map(|path| {
            let filtered_paths_clone = Arc::clone(&filtered_paths);
            let area_clone = area.clone();
            thread::spawn(move || {
                if !is_file_far_away_from_area(&path, &area_clone, distance) {
                    let mut filtered_paths = filtered_paths_clone.lock().unwrap();
                    filtered_paths.push(path);
                }
            })
        })
        .collect();

    for thread in threads {
        thread.join().unwrap();
    }
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
