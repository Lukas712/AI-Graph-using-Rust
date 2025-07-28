const EARTH_RADIUS: f64 = 6371.0;

/// Função para calcular a distância entre duas cidades usando a fórmula de Haversine.
/// A fórmula é:
/// d = 2 * R * arcsin(sqrt(sin²((lat2 - lat1)/2) + cos(lat1) * cos(lat2) * sin²((lon2 - lon1)/2)))
/// Onde R é o raio da Terra (aproximadamente 6371 km).
pub fn calculate_distance_value(lat_origin: f64, lon_origin: f64,lat_destiny: f64, lon_destiny: f64) -> f64 {
    let d_lat = (lat_destiny - lat_origin).to_radians();
    let d_lon = (lon_destiny - lon_origin).to_radians();
    let lat1_rad = lat_origin.to_radians();
    let lat2_rad = lat_destiny.to_radians();

    let a =
        f64::sin(d_lat / 2.0).powi(2) +
        f64::cos(lat1_rad) * f64::cos(lat2_rad) * f64::sin(d_lon / 2.0).powi(2);

    let c = 2.0 * f64::atan2(a.sqrt(), (1.0 - a).sqrt());
    return EARTH_RADIUS * c;
}

/// Função para calcular o nível de uma cidade com base na distância heurística.
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
