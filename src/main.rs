use gpx_geo_filter::{cli::get_cli_arguments, copy_gpx_files, filter_tracks};

fn main() {
    let config = get_cli_arguments();

    let files = filter_tracks(
        &config
            .folder
            .to_str()
            .expect("The folder can not be parsed"),
        config.first_lat,
        config.first_lon,
        config.second_lat,
        config.second_lon,
        config.distance,
        config.threads,
    );

    match config.copy_to {
        Some(path) => {
            println!("Copying filtered files to {:?}", path.display());
            copy_gpx_files(files, &path.to_str().expect("Invalid output path"));
        }
        _ => (),
    }
}
