#[derive(Clone, PartialEq, Debug)]
pub struct Coordinate {
    pub latitude: f32,
    pub longitude: f32,
}
impl Coordinate {
    pub fn new(latitude: f32, longitude: f32) -> Coordinate {
        Coordinate {
            latitude,
            longitude,
        }
    }
}

impl Copy for Coordinate {}

#[derive(Clone, PartialEq, Debug)]

pub struct SquaredFilter {
    pub top_left: Coordinate,
    pub top_right: Coordinate,
    pub bottom_right: Coordinate,
    pub bottom_left: Coordinate,
}

impl SquaredFilter {
    pub fn new(a: Coordinate, b: Coordinate) -> SquaredFilter {
        SquaredFilter {
            top_left: Coordinate::new(b.latitude, a.longitude),
            top_right: b,
            bottom_right: Coordinate::new(a.latitude, b.longitude),
            bottom_left: a,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_coordinate() {
        let longitude = 47.3753910;
        let latitude = 8.5763870;
        let coordinate = Coordinate::new(latitude, longitude);
        assert_eq!(coordinate.longitude, longitude);
        assert_eq!(coordinate.latitude, latitude);
    }

    #[test]
    fn create_filter() {
        let longitude_a = 47.3753910;
        let latitude_a = 8.5763870;
        let coordinate_a = Coordinate::new(latitude_a, longitude_a);

        let longitude_b = 87.1634560;
        let latitude_b = 20.5763675;
        let coordinate_b = Coordinate::new(latitude_b, longitude_b);

        let area = SquaredFilter::new(coordinate_a, coordinate_b);

        assert_eq!(area.top_left, Coordinate::new(latitude_b, longitude_a));
        assert_eq!(area.bottom_right, Coordinate::new(latitude_a, longitude_b));
        assert_eq!(area.top_right, coordinate_b);
        assert_eq!(area.bottom_left, coordinate_a);
    }
}
