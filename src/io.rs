use std::{fs::read_dir, fs::File, io::BufReader, path::PathBuf};

use quick_xml::{events::Event, reader::Reader};

use crate::model::Coordinate;

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

pub fn read_xml_file(path: &PathBuf) -> Vec<Coordinate> {
    let file = File::open(path).unwrap();
    let buff_reader = BufReader::new(file);

    let mut reader = Reader::from_reader(Box::new(buff_reader));
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut coordinates: Vec<Coordinate> = Vec::new();
    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"trkpt" => {
                    let longitude: Option<f32> = e
                        .attributes()
                        .filter_map(|a| a.ok())
                        .find(|attr| attr.key == quick_xml::name::QName(b"lon"))
                        .and_then(|attr| String::from_utf8_lossy(&attr.value).parse().ok());
                    let latitude: Option<f32> = e
                        .attributes()
                        .filter_map(|a| a.ok())
                        .find(|attr| attr.key == quick_xml::name::QName(b"lat"))
                        .and_then(|attr| String::from_utf8_lossy(&attr.value).parse().ok());
                    if latitude.is_some() && longitude.is_some() {
                        coordinates.push(Coordinate::new(latitude.unwrap(), longitude.unwrap()))
                    }
                }
                _ => (),
            },
            _ => (), // ignore other xml events
        }
        buf.clear(); // clear memory
    }
    coordinates
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &str = "test/files/9244476879.gpx";
    #[test]
    fn test_read_xml() {
        let path = PathBuf::from(TEST_FILE);
        let coordinates = read_xml_file(&path);
        assert_eq!(2245, coordinates.len());
    }
}
