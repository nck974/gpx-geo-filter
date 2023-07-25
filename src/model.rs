#[derive(Clone, PartialEq, Debug)]
pub struct Coordinate {
    pub longitude: f32,
    pub latitude: f32,
}
impl Coordinate {
    pub fn new(longitude: f32, latitude: f32) -> Coordinate {
        Coordinate {
            longitude,
            latitude,
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
            top_left: Coordinate::new(a.longitude, b.latitude),
            top_right: b,
            bottom_right: Coordinate::new(b.longitude, a.latitude),
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
        let coordinate = Coordinate::new(longitude, latitude);
        assert_eq!(coordinate.longitude, longitude);
        assert_eq!(coordinate.latitude, latitude);
    }

    #[test]
    fn create_filter() {
        let longitude_a = 47.3753910;
        let latitude_a = 8.5763870;
        let coordinate_a = Coordinate::new(longitude_a, latitude_a);

        let longitude_b = 87.1634560;
        let latitude_b = 20.5763675;
        let coordinate_b = Coordinate::new(longitude_b, latitude_b);

        let area = SquaredFilter::new(coordinate_a, coordinate_b);

        assert_eq!(area.top_left, Coordinate::new(longitude_a, latitude_b));
        assert_eq!(area.bottom_right, Coordinate::new(longitude_b, latitude_a));
        assert_eq!(area.top_right, coordinate_b);
        assert_eq!(area.bottom_left, coordinate_a);
    }
}
