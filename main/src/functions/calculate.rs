const EARTH_RADIUS: f64 = 6371.0;

pub fn calculate_distance_value(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();

    let a =
        f64::sin(d_lat / 2.0).powi(2) +
        f64::cos(lat1_rad) * f64::cos(lat2_rad) * f64::sin(d_lon / 2.0).powi(2);

    let c = 2.0 * f64::atan2(a.sqrt(), (1.0 - a).sqrt());
    return EARTH_RADIUS * c;
}

pub fn calculate_level(
    heuristic_origin: f64,
    heuristic_destiny: f64,
    number_of_levels: usize
) -> usize {
    let proportion = heuristic_origin / heuristic_destiny;
    let level = (proportion * (number_of_levels as f64) - (number_of_levels as f64))
        .abs()
        .ceil() as usize;

    return level;
}
