use gpx_geo_filter::{filter_tracks, copy_gpx_files};

static DIRECTORY: &str = "samples";
// static DIRECTORY: &str = r"C:\Users\nck\Development\py-geo-locator\gps_data";
static OUTPUT: &str = r"output";

fn main() {
    let first_lat = 49.454470;
    let first_lon = 10.954986;
    let second_lat = 49.506443;
    let second_lon = 11.030173;

    let distance = 300.0;
    let threads: usize = 12;

    let files = filter_tracks(
        DIRECTORY, first_lat, first_lon, second_lat, second_lon, distance, threads,
    );

    copy_gpx_files(files, OUTPUT);
}
