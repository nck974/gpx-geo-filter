use std::{
    fs::read_dir,
    path::PathBuf,
};


pub fn collect_data(directory: &str) -> Vec<PathBuf> {
    println!("Reading files from {directory}!");

    let files = read_dir(directory).unwrap_or_else(|_| panic!("Directory could not be found!"));
    let mut found_files: Vec<PathBuf> = Vec::new();
    for file in files {
        match file {
            Ok(file) => {
                let file_path = file.path();
                found_files.push(file_path);
            }
            Err(err) => {
                println!("Error reading file: {err}");
            }
        }
    }
    found_files
}
