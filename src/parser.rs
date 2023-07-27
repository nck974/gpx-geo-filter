use regex::Regex;

use crate::model::Coordinate;

/// Compiling a regex in first place seems to improve the performance by 20x
/// in 200 files
pub fn compile_coordinate_regex() -> Regex {
    let pattern = r#"<trkpt\s+lat="(-?[\d.]+)"\s+lon="(-?[\d.]+)"\s*>"#;
    Regex::new(pattern).unwrap()
}

/// The pre-filtering expects to find the trackpoints in a single line with just two parameters
pub fn extract_first_coordinate_from_text(re: Regex, text: &str) -> Option<Coordinate> {
    // Use the regex to extract latitude and longitude
    if let Some(captures) = re.captures(text) {
        let latitude: f32 = captures[1].parse().unwrap();
        let longitude: f32 = captures[2].parse().unwrap();

        return Some(Coordinate::new(latitude, longitude));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_text() {
        let input = r#"<trkpt lat="48.7890140" lon="9.2344190">"#;
        let coordinate = Coordinate::new(48.7890140, 9.2344190);
        let re = compile_coordinate_regex();
        assert_eq!(
            Some(coordinate),
            extract_first_coordinate_from_text(re, input)
        );
    }
    #[test]
    fn extract_text_negative() {
        let input = r#"<trkpt lat="-48.7890140" lon="-9.2344190">"#;
        let coordinate = Coordinate::new(-48.7890140, -9.2344190);
        let re = compile_coordinate_regex();
        assert_eq!(
            Some(coordinate),
            extract_first_coordinate_from_text(re, input)
        );
    }
}
