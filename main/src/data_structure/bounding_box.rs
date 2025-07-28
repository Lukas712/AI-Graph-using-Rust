use crate::data_structure::city::City;

/// Estrutura para armazenar uma caixa delimitadora (bounding box) que define uma área geográfica.
/// 
/// Atributos:
/// - `south`: Latitude mínima (sul).
/// - `north`: Latitude máxima (norte).
/// - `west`: Longitude mínima (oeste).
/// - `east`: Longitude máxima (leste).
#[derive(Debug, Clone)]
pub struct BoundingBox {
    south: f64,
    north: f64,
    west: f64,
    east: f64,
}

impl BoundingBox {
    /// Cria uma nova caixa delimitadora a partir de duas cidades, com uma margem adicional.
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

    /// Retorna as coordenadas no formato aceito pela API Overpass.
    pub fn to_overpass_format(&self) -> String {
        format!("{},{},{},{}", self.south, self.west, self.north, self.east)
    }
}
