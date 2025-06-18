const EARTH_RADIUS: f64 = 6371.0;

pub struct City {
    name: String,
    heuristic_value: f64,
    visited: bool,
    latitude: f64,
    longitude: f64,
    level: usize,
}

impl City {
    pub fn new(name: String, latitude: f64, longitude: f64, latitude_destiny: f64, longitude_destiny: f64) -> City {
        
        let heuristic = calculate_heuristic_value(latitude, longitude, latitude_destiny, longitude_destiny);
        
        City {
            name,
            heuristic_value: heuristic,
            visited: false,
            latitude,
            longitude,
            level: 0,
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
            "City(name: {},heuristic_value: {}, visited: {}, latitude: {}, longitude: {}, nível: {})",
            self.name ,self.heuristic_value, self.visited, self.latitude, self.longitude, self.level
        )
    }

    pub fn get_level(&self) -> usize {
        self.level
    }

    pub fn set_level(&mut self, level: usize) {
        self.level = level;
    }

}

fn calculate_heuristic_value(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {

        let d_lat = (lat2 - lat1).to_radians();
        let d_lon = (lon2 - lon1).to_radians();
        let lat1_rad = lat1.to_radians();
        let lat2_rad = lat2.to_radians();

        let a = f64::sin(d_lat / 2.0).powi(2)
            + f64::cos(lat1_rad)
            * f64::cos(lat2_rad)
            * f64::sin(d_lon / 2.0).powi(2);
        
        let c = 2.0 * f64::atan2(a.sqrt(), (1.0 - a).sqrt());
        EARTH_RADIUS * c
}

pub fn calculate_level(heuristic_origin: f64, heuristic_destiny: f64, number_of_levels: usize) -> usize {
    let proportion = heuristic_origin / heuristic_destiny;
    let level = ((((proportion * (number_of_levels as f64))-number_of_levels as f64)).abs()).ceil() as usize;

    level
}