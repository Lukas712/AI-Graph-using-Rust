use rand::seq::index;
use rand::Rng;

use super::graph::GraphStructure;

use crate::functions::calculate::{calculate_distance_value, calculate_level};
use crate::functions::get_from_api::{get_all_cities_from_bounding_box, get_city_coordinates};
use crate::data_structure::{bounding_box::BoundingBox, city::City};
use crate::functions::between_cities::is_between_bounding_box;

const MARGIN_DEGREES: f64 = 0.5;
const MIN_BANDS: usize = 2;
const MARGIN_KM: f64 = 5.0;



/// A estrutura de dados `Map` representa um mapa, contendo um grafo de cidades e caminhos.
/// 
/// Atributos:
/// - `graph`: Um grafo que representa as cidades e os caminhos entre elas.
/// - `bbox`: Uma caixa delimitadora que define a área geográfica do mapa.
/// - `number_of_levels`: O número de níveis no grafo, usado para organizar as
pub struct Map {
    graph: GraphStructure,
    bbox: Option<BoundingBox>,
    number_of_levels: Option<usize>,
}

impl Map {
    /// Construtor para criar um novo mapa a partir de duas cidades: a cidade de origem e a cidade objetivo.
    pub fn new(
        root_city: String,
        objective_city: String
    ) -> Result<Map, Box<dyn std::error::Error>> {

        let (root_city_lat, root_city_lon) = get_city_coordinates(&root_city)?;

        let (objective_city_lat, objective_city_lon) = get_city_coordinates(&objective_city)?;

        let origin = City::new(
            root_city.clone(),
            root_city_lat,
            root_city_lon,
            objective_city_lat,
            objective_city_lon,
        );

        let objective_city = City::new(
            objective_city.clone(),
            objective_city_lat,
            objective_city_lon,
            objective_city_lat,
            objective_city_lon,
        );

        let graph = GraphStructure::new();
        let mut map = Map {
            graph,
            bbox: None,
            number_of_levels: None,
        };
        map.calculate_number_of_levels(&origin.clone());
        
        map.insert_origin(origin.clone());
        map.insert_objective(objective_city.clone(), origin.clone());

        map.create_bounding_box();

        map.create_map_cities()?;
        map.create_map_paths();

        Ok(map)
    }

    /// Retorna uma referência ao grafo contido no mapa.
    pub fn get_graph(&self) -> &GraphStructure {
        return &self.graph
    }

    /// Retorna o número de níveis do grafo.
    pub fn get_number_of_levels(&self) -> usize {
        return self.number_of_levels.unwrap();
    }

    /// Insere uma nova cidade no grafo, se ela estiver dentro da caixa delimitadora do mapa.
    pub fn insert_city(&mut self, city: City){
        let index_root = self.graph.get_root().unwrap();
        let index_objective = self.graph.get_objective().unwrap();

        if(is_between_bounding_box(self.get_graph(),index_root, index_objective, &city)) {

            if(city.get_heuristic_value()< self.graph.get_city(index_root).unwrap().get_heuristic_value()) {
                let level = self.calculate_level(&city, self.graph.get_city(index_root).unwrap());
                self.graph.insert_city(level, city);
            }
        }
    }

    ///Calcula o nível de uma cidade com base em sua heurística e a heurística da cidade de origem.
    pub fn calculate_level(
        &self,
        city: &City,
        origin: &City,
    ) -> usize {
        return calculate_level(city.get_heuristic_value(), origin.get_heuristic_value(), self.number_of_levels.unwrap());
    }

    /// Insere a cidade de origem no grafo e define-a como raiz.
    pub fn insert_origin(&mut self, origin: City) {
        let level = self.calculate_level(&origin, &origin);
        let root = self.graph.insert_city(level, origin);
        self.graph.set_root(root);
    }

    /// Insere a cidade objetivo no grafo e define-a como objetivo.
    pub fn insert_objective(&mut self, objective: City, origin: City) {
        let level = self.calculate_level(&objective, &origin);
        let objective = self.graph.insert_city(level, objective);
        self.graph.set_objective(objective);
    }

    /// Cria uma caixa delimitadora com base nas cidades de origem e objetivo.
    fn create_bounding_box(&mut self){
        let index_root = self.graph.get_root().unwrap();
        let index_objective = self.graph.get_objective().unwrap();
        self.bbox = Some(BoundingBox::from_cities(self.graph.get_city(index_root).unwrap(), self.graph.get_city(index_objective).unwrap(), MARGIN_DEGREES));
    }

    /// Calcula o número de níveis do grafo com base na distância heurística da cidade de origem.
    fn calculate_number_of_levels(&mut self, origin: &City) {
        let distance: f64 = origin.get_heuristic_value();
        
        let mut current_band_size = MARGIN_KM;
        let mut n_bands = MIN_BANDS;
        
        while (current_band_size * n_bands as f64) < distance {
            current_band_size += MARGIN_KM;
            n_bands += 1;
        }
        self.number_of_levels = Some(n_bands);
    }

    /// Cria as cidades do mapa a partir da API [`Overpass`], inserindo as cidades que estão entre as cidades `Origem` e `Destino` dentro da [`BoundingBox`].
    fn create_map_cities(&mut self) -> Result<(), Box<dyn std::error::Error>> 
    {
        let all_cities = get_all_cities_from_bounding_box(
            self.bbox.as_ref().unwrap().clone(),
            self.graph.get_city(self.graph.get_root().unwrap()).unwrap().get_name().to_string(),
            self.graph.get_city(self.graph.get_objective().unwrap()).unwrap().get_name().to_string(),
        )?;
        
        for city in all_cities {
            let newCity = City::new(
                city.0.clone(),
                city.1,
                city.2,
                self.graph.get_city(self.graph.get_objective().unwrap()).unwrap().get_latitude(),
                self.graph.get_city(self.graph.get_objective().unwrap()).unwrap().get_longitude(),
            );
            if(newCity.get_name() != self.graph.get_city(self.graph.get_root().unwrap()).unwrap().get_name() && newCity.get_name() != self.graph.get_city(self.graph.get_objective().unwrap()).unwrap().get_name()) {
                self.insert_city(newCity.clone());
            }
        }

        Ok(())
    }

    /// Cria os caminhos entre as cidades do mapa, adicionando arestas ao grafo.
    fn create_map_paths(&mut self) {
        for level in 0..self.get_number_of_levels() {
            if let Some(cities) = self.graph.get_entire_level(level) {
                for city in cities {
                    if(level <= self.get_number_of_levels() - 1) {
                        if let Some(next_cities) = self.graph.get_entire_level(level + 1) {
                            for next_city in next_cities {
                                let distance = calculate_distance_value(
                                    self.graph.get_city(city).unwrap().get_latitude(),
                                    self.graph.get_city(city).unwrap().get_longitude(),
                                    self.graph.get_city(next_city).unwrap().get_latitude(),
                                    self.graph.get_city(next_city).unwrap().get_longitude(),
                                );

                                let mut rng = rand::thread_rng();
                                let margin_error: f64 = rng.gen_range(0.15..0.3);
                                // let margin_error = 0.0;
                                self.graph.add_edge(city, next_city, distance* (1.0 + margin_error));
                            }
                        }
                    }
                }
            }
        }
    }

    /// Imprime as cidades do grafo organizadas por níveis.
    pub fn print_graph_by_levels(&self) {
        println!("Número de níveis: {}", self.get_number_of_levels());
        for level in 0..self.get_number_of_levels() + 1{
            println!("Nível {}:", level);

            if let Some(cities) = self.graph.get_entire_level(level) {
                for city in cities {
                    println!(
                        "  Cidade: {}, Heurística: {}, Latitude: {}, Longitude: {}",
                        self.graph.get_city(city).unwrap().get_name(),
                        self.graph.get_city(city).unwrap().get_heuristic_value(),
                        self.graph.get_city(city).unwrap().get_latitude(),
                        self.graph.get_city(city).unwrap().get_longitude()
                    );
                }
            } else {
                println!("  Nenhuma cidade neste nível.");
            }
        }
        println!();
    }


}