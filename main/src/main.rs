mod data_structure;
use data_structure::city::City;
use data_structure::city::calculate_level;

use petgraph::graph::Graph;
use reqwest::blocking::Client;
use serde_json::Value;
use std::error::Error;

use crate::data_structure::city;

#[derive(Debug)]
struct BoundingBox {
    south: f64,
    north: f64,
    west: f64,
    east: f64,
}

impl BoundingBox {
    fn from_cities(city1: (f64, f64), city2: (f64, f64), margin_degrees: f64) -> Self {
        let (lat1, lon1) = city1;
        let (lat2, lon2) = city2;
        
        BoundingBox {
            south: lat1.min(lat2) - margin_degrees,
            north: lat1.max(lat2) + margin_degrees,
            west: lon1.min(lon2) - margin_degrees,
            east: lon1.max(lon2) + margin_degrees,
        }
    }
    
    fn to_overpass_format(&self) -> String {
        format!("{},{},{},{}", self.south, self.west, self.north, self.east)
    }
}

fn get_city_coordinates(city_name: &str) -> Result<(f64, f64), Box<dyn Error>> {
    let client = Client::new();
    
    let query = format!(
        r#"[out:json];
        area["name"="Brasil"]->.searchArea;
        (
          node["place"~"city|town"]["name"="{}"](area.searchArea);
          way["place"~"city|town"]["name"="{}"](area.searchArea);
          relation["place"~"city|town"]["name"="{}"](area.searchArea);
        );
        out center;"#,
        city_name, city_name, city_name
    );

    let res = client
        .post("https://overpass-api.de/api/interpreter")
        .body(query)
        .send()?
        .json::<Value>()?;

    if let Some(elements) = res["elements"].as_array() {
        for el in elements {
            let (lat, lon) = match el["type"].as_str() {
                Some("node") => (
                    el["lat"].as_f64().ok_or("Latitude não encontrada")?,
                    el["lon"].as_f64().ok_or("Longitude não encontrada")?,
                ),
                Some("way") | Some("relation") => (
                    el["center"]["lat"].as_f64().ok_or("Latitude não encontrada")?,
                    el["center"]["lon"].as_f64().ok_or("Longitude não encontrada")?,
                ),
                _ => continue,
            };
            return Ok((lat, lon));
        }
    }

    Err(format!("Cidade '{}' não encontrada", city_name).into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    let (lat_dest, lon_dest) = get_city_coordinates("Rio de Janeiro")?;
    let (lat_orig, lon_orig) = get_city_coordinates("São Paulo")?;
    
    let mut city_orig = City::new(
        "São Paulo".to_string(),
        lat_orig,
        lon_orig,
        lat_dest,
        lon_dest,
    );
    
    let mut city_dest = City::new(
        "Rio de Janeiro".to_string(),
        lat_dest,
        lon_dest,
        lat_dest,
        lon_dest,
    );
    
    let margin = 0.5;
    let bbox = BoundingBox::from_cities((lat_orig, lon_orig), (lat_dest, lon_dest), margin);
    println!("Bounding Box: {:?}", bbox);

    let min_bands = 10;
    let max_band_km = 50.0;
    let distance : f64 = city_orig.get_heuristic_value();
    let n_bands = (distance / max_band_km).ceil() as usize;
    let n_bands = n_bands.max(min_bands);
    let band_width = distance / (n_bands as f64);
    println!("Número de faixas: {}", n_bands);
    println!("Largura de cada faixa: {:.2} km", band_width);

    city_orig.set_level(calculate_level(city_orig.get_heuristic_value(),city_orig.get_heuristic_value(), n_bands));

    city_dest.set_level(calculate_level(city_dest.get_heuristic_value(),city_orig.get_heuristic_value(), n_bands));


    let query = format!(
        r#"[out:json];
        node
          ["place"~"city|town|village|hamlet"]
          ({});
        out;"#,
        bbox.to_overpass_format()
    );

    println!("Executando query:\n{}", query);

    let res = client
        .post("https://overpass-api.de/api/interpreter")
        .body(query)
        .send()?
        .json::<Value>()?;

    let mut graph: Graph<City, f32> = Graph::new();

    if let Some(elements) = res["elements"].as_array() {
        for el in elements {
            let lat = el["lat"].as_f64();
            let lon = el["lon"].as_f64();

            if let (Some(latitude), Some(longitude)) = (lat, lon) {
                if let Some(tags) = el["tags"].as_object() {
                    if let Some(name_value) = tags.get("name") {
                        let name = name_value.as_str().unwrap_or("Unknown").to_string();
                        if(latitude != lat_orig && longitude != lon_orig && latitude != lat_dest && longitude != lon_dest) {
                            let mut city = City::new(name, latitude, longitude, lat_dest, lon_dest);
                            if(city.get_heuristic_value() < city_orig.get_heuristic_value()) {
                                city.set_level(calculate_level(city.get_heuristic_value(), city_orig.get_heuristic_value(), n_bands));
                                graph.add_node(city);
                            }
                        }
                    }
                }
            }
        }
    }

    graph.add_node(city_orig);
    graph.add_node(city_dest);

    println!("Número total de cidades inseridas no grafo: {}", graph.node_count());

    for node in graph.node_indices() {
        
        let city = graph[node].to_string();
        println!("{}", city);
    }

    Ok(())
}
