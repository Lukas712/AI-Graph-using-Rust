use crate::functions;
use functions::calculate::calculate_distance_value;

/** Estrutura que representa uma cidade no grafo.

Atributos:
- `name`: Nome da cidade.
- `heuristic_value`: Valor heurístico da cidade, usado em algoritmos de busca.
- `visited`: Indica se a cidade foi visitada durante a busca.
- `latitude`: Latitude da cidade.
- `longitude`: Longitude da cidade.
*/
#[derive(Clone, PartialEq)]
pub struct City {
    name: String,
    heuristic_value: f64,
    latitude: f64,
    longitude: f64,
}

impl City {
    /// Construtor para criar uma nova cidade com nome, coordenadas e valores heurísticos.
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
            latitude,
            longitude,
        }
    }

    /// Retorna o valor heurístico da cidade.
    pub fn get_heuristic_value(&self) -> f64 {
        self.heuristic_value
    }

    /// Retorna a latitude da cidade.
    pub fn get_latitude(&self) -> f64 {
        self.latitude
    }

    /// Retorna a longitude da cidade.
    pub fn get_longitude(&self) -> f64 {
        self.longitude
    }

    /// Define o valor heurístico da cidade.
    pub fn set_heuristic_value(&mut self, heuristic_value: f64) {
        self.heuristic_value = heuristic_value;
    }

    /// Define a latitude e longitude da cidade.
    pub fn set_latitude(&mut self, latitude: f64) {
        self.latitude = latitude;
    }

    /// Define a longitude da cidade.
    pub fn set_longitude(&mut self, longitude: f64) {
        self.longitude = longitude;
    }

    /// Retorna o nome da cidade.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Define o nome da cidade.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Retorna uma representação em string da cidade.
    pub fn to_string(&self) -> String {
        format!(
            "City(name: {},heuristic_value: {}, latitude: {}, longitude: {})",
            self.name,
            self.heuristic_value,
            self.latitude,
            self.longitude,
        )
    }
}
