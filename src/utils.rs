use crate::model::{Coordinate, SquaredFilter};

pub fn is_point_more_than_x_distance_from_filter(
    area: &SquaredFilter,
    point: &Coordinate,
    distance: f32,
) -> bool {
    let closest_longitude = f32::max(
        area.top_left.longitude,
        f32::min(point.longitude, area.top_right.longitude),
    );
    let closest_latitude = f32::max(
        area.top_left.latitude,
        f32::min(point.latitude, area.bottom_right.latitude),
    );

    let closest_point = Coordinate::new(closest_longitude, closest_latitude);

    match calculate_distance_between_two_coordinates_in_km(&point, &closest_point) > distance {
        true => return true,
        false => return false,
    }
}

/// Check if the provided point is within the boundaries of the area
pub fn is_point_in_area(area: &SquaredFilter, point: &Coordinate) -> bool {
    point.longitude >= area.top_left.longitude
        && point.longitude < area.top_right.longitude
        && point.latitude < area.top_left.latitude
        && point.latitude >= area.bottom_left.latitude
}

/// Constants are extracted from:
/// https://stackoverflow.com/questions/1253499/simple-calculations-for-working-with-lat-lon-and-km-distance
fn calculate_distance_between_two_coordinates_in_km(a: &Coordinate, b: &Coordinate) -> f32 {
    const LAT_TO_KM: f32 = 110.574;
    const LON_TO_KM: f32 = 111.320;

    let delta_latitude_km = (a.latitude - b.latitude) * LAT_TO_KM;
    let delta_longitude_km =
        (a.longitude - b.longitude) * LON_TO_KM * a.latitude.to_radians().cos();

    let distance_km = (delta_latitude_km.powi(2) + delta_longitude_km.powi(2)).sqrt();
    // println!("Distance is {}", distance_km);
    distance_km
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_between_two_points_in_km() {
        let coordinate_a = Coordinate::new(47.3753910, 20.5763675);
        let coordinate_b = Coordinate::new(87.3753910, 20.5763675);

        assert_eq!(
            4168.7314,
            calculate_distance_between_two_coordinates_in_km(&coordinate_a, &coordinate_b)
        );
        assert_ne!(
            4168.7310,
            calculate_distance_between_two_coordinates_in_km(&coordinate_a, &coordinate_b)
        );
    }

    #[test]
    fn distance_to_area_same_axis_x() {
        let point = Coordinate::new(100.0, 80.0);
        let area = SquaredFilter::new(Coordinate::new(40.0, 40.0), Coordinate::new(80.0, 80.0));

        assert_eq!(
            false,
            is_point_more_than_x_distance_from_filter(&area, &point, 387.0)
        );
        assert_eq!(
            true,
            is_point_more_than_x_distance_from_filter(&area, &point, 386.0)
        );
    }
    #[test]
    fn distance_to_area_same_axis_y() {
        let point = Coordinate::new(40.0, 100.0);
        let area = SquaredFilter::new(Coordinate::new(40.0, 40.0), Coordinate::new(80.0, 80.0));

        assert_eq!(
            false,
            is_point_more_than_x_distance_from_filter(&area, &point, 2212.0)
        );
        assert_eq!(
            true,
            is_point_more_than_x_distance_from_filter(&area, &point, 2211.0)
        );
    }
    #[test]
    fn distance_to_area_middle() {
        let point = Coordinate::new(90.0, 90.0);
        let area = SquaredFilter::new(Coordinate::new(40.0, 40.0), Coordinate::new(80.0, 80.0));

        assert_eq!(
            false,
            is_point_more_than_x_distance_from_filter(&area, &point, 1106.0)
        );
        assert_eq!(
            true,
            is_point_more_than_x_distance_from_filter(&area, &point, 1105.0)
        );
    }    
    #[test]
    fn point_is_in_area() {
        let point = Coordinate::new(45.0, 45.0);
        let area = SquaredFilter::new(Coordinate::new(40.0, 40.0), Coordinate::new(80.0, 80.0));

        assert_eq!(
            true,
            is_point_in_area(&area, &point)
        );
    }    
    #[test]
    fn point_is_not_in_area() {
        let point = Coordinate::new(91.0, 91.0);
        let area = SquaredFilter::new(Coordinate::new(40.0, 40.0), Coordinate::new(80.0, 80.0));

        assert_eq!(
            false,
            is_point_in_area(&area, &point)
        );
    }
}
