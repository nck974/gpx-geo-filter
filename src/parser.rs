use regex::Regex;

use crate::model::Coordinate;

/// The pre-filtering expects to find the trackpoints in a single line with just two parameters
pub fn extract_first_coordinate_from_text(text: &str) -> Option<Coordinate> {
    let pattern = r#"<trkpt\s+lat="(-?[\d.]+)"\s+lon="(-?[\d.]+)"\s*>"#;
    let re = Regex::new(pattern).unwrap();

    // Use the regex to extract latitude and longitude
    if let Some(captures) = re.captures(text) {
        let longitude: f32 = captures[1].parse().unwrap();
        let latitude: f32 = captures[2].parse().unwrap();

        return Some(Coordinate {
            longitude,
            latitude,
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_text() {
        // Input string containing the <trkpt> element
        let input = r#"<trkpt lat="48.7890140" lon="9.2344190">"#;
        let coordinate = Coordinate::new(48.7890140, 9.2344190);
        assert_eq!(Some(coordinate), extract_first_coordinate_from_text(input));
    }
    #[test]
    fn extract_text_negative() {
        // Input string containing the <trkpt> element
        let input = r#"<trkpt lat="-48.7890140" lon="-9.2344190">"#;
        let coordinate = Coordinate::new(-48.7890140, -9.2344190);
        assert_eq!(Some(coordinate), extract_first_coordinate_from_text(input));
    }
}
