use crate::functions;
use functions::calculate::calculate_distance_value;

#[derive(Clone, PartialEq)]
pub struct City {
    name: String,
    heuristic_value: f64,
    visited: bool,
    latitude: f64,
    longitude: f64,
}

impl City {
    pub fn new(
        name: String,
        latitude: f64,
        longitude: f64,
        latitude_destiny: f64,
        longitude_destiny: f64
    ) -> City {
        let heuristic = calculate_distance_value(
            latitude,
            longitude,
            latitude_destiny,
            longitude_destiny
        );

        City {
            name,
            heuristic_value: heuristic,
            visited: false,
            latitude,
            longitude,
        }
    }

    pub fn get_heuristic_value(&self) -> f64 {
        self.heuristic_value
    }

    pub fn is_visited(&self) -> bool {
        self.visited
    }

    pub fn get_latitude(&self) -> f64 {
        self.latitude
    }

    pub fn get_longitude(&self) -> f64 {
        self.longitude
    }

    pub fn set_visited(&mut self, visited: bool) {
        self.visited = visited;
    }
    pub fn set_heuristic_value(&mut self, heuristic_value: f64) {
        self.heuristic_value = heuristic_value;
    }

    pub fn set_latitude(&mut self, latitude: f64) {
        self.latitude = latitude;
    }

    pub fn set_longitude(&mut self, longitude: f64) {
        self.longitude = longitude;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn to_string(&self) -> String {
        format!(
            "City(name: {},heuristic_value: {}, visited: {}, latitude: {}, longitude: {})",
            self.name,
            self.heuristic_value,
            self.visited,
            self.latitude,
            self.longitude,
        )
    }
}
