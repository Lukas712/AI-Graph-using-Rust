use reqwest::blocking::Client;
use serde_json::Value;
use std::error::Error;

use crate::data_structure::bounding_box::BoundingBox;

/// Obtém as coordenadas de uma cidade a partir do nome da cidade usando a API Overpass.
/// 
/// Retorna um `Result` com as coordenadas (latitude, longitude) ou um erro se a cidade não for encontrada.
pub fn get_city_coordinates(city_name: &str) -> Result<(f64, f64), Box<dyn Error>> {
    println!("Obtendo a cidade {}...", city_name);
    let start_time = std::time::Instant::now();
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
        city_name,
        city_name,
        city_name
    );

    let res = client
        .post("https://overpass-api.de/api/interpreter")
        .body(query)
        .send()?
        .json::<Value>()?;

    if let Some(elements) = res["elements"].as_array() {
        for el in elements {
            let (lat, lon) = match el["type"].as_str() {
                Some("node") =>
                    (
                        el["lat"].as_f64().ok_or("Latitude não encontrada")?,
                        el["lon"].as_f64().ok_or("Longitude não encontrada")?,
                    ),
                Some("way") | Some("relation") =>
                    (
                        el["center"]["lat"].as_f64().ok_or("Latitude não encontrada")?,
                        el["center"]["lon"].as_f64().ok_or("Longitude não encontrada")?,
                    ),
                _ => {
                    continue;
                }
            };
            println!(
                "Tempo gasto para obter a cidade: {:.2?}",
                start_time.elapsed()
            );
            println!();
            return Ok((lat, lon));
        }
    }
    println!(
        "Tempo gasto: {:.2?}",
        start_time.elapsed()
    );
    println!();
    Err(format!("Cidade '{}' não encontrada", city_name).into())
}

/// Obtém todas as cidades dentro de uma caixa delimitadora (bounding box) usando a API Overpass.
/// Retorna um `Result` com um vetor de tuplas contendo o nome da cidade, latitude e longitude, ou um erro se não for possível obter as cidades.
pub fn get_all_cities_from_bounding_box(
    bbox: BoundingBox,
    root: String,
    objective: String,
) -> Result<Vec<(String, f64, f64)>, Box<dyn Error>> {
    let mut start_time = std::time::Instant::now();
    println!("Obtendo as cidades entre {} e {}...", root, objective);
    let client = Client::new();

    let query = format!(
        r#"[out:json];
        node
          ["place"~"city|town|village"]
          ({});
        out;"#,
        bbox.to_overpass_format()
    );

    let res = client
        .post("https://overpass-api.de/api/interpreter")
        .body(query)
        .send()?
        .json::<Value>()?;

    let mut cities = Vec::new();

    if let Some(elements) = res["elements"].as_array() {
        for el in elements {
            if let (Some(lat), Some(lon)) = (el["lat"].as_f64(), el["lon"].as_f64()) {
                if let Some(tags) = el["tags"].as_object() {
                    if let Some(name_value) = tags.get("name") {
                        let name = name_value.as_str().unwrap_or("Unknown").to_string();
                        cities.push((name, lat, lon));
                    }
                }
            }
        }
    }
    println!(
        "Tempo gasto para obter as cidades: {:.2?}",
        start_time.elapsed()
    );
    println!();
    Ok(cities)
}