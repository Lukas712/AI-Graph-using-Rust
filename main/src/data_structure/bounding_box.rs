use crate::data_structure::city::City;

#[derive(Debug, Clone)]
pub struct BoundingBox {
    south: f64,
    north: f64,
    west: f64,
    east: f64,
}

impl BoundingBox {
    pub fn from_cities(city1: &City, city2: &City, margin_degrees: f64) -> Self {
        let (lat1, lon1) = (city1.get_latitude(), city1.get_longitude());
        let (lat2, lon2) = (city2.get_latitude(), city2.get_longitude());

        BoundingBox {
            south: lat1.min(lat2) - margin_degrees,
            north: lat1.max(lat2) + margin_degrees,
            west: lon1.min(lon2) - margin_degrees,
            east: lon1.max(lon2) + margin_degrees,
        }
    }

    pub fn to_overpass_format(&self) -> String {
        format!("{},{},{},{}", self.south, self.west, self.north, self.east)
    }
}
