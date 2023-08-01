use std::path::PathBuf;

pub struct GpxGeoFilterConfig {
    pub first_lat: f32,
    pub first_lon: f32,
    pub second_lat: f32,
    pub second_lon: f32,
    pub distance: f32,
    pub folder: PathBuf,
    pub threads: usize,
    pub copy_to: Option<PathBuf>,
}
