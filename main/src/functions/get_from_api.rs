use reqwest::blocking::Client;
use serde_json::Value;
use std::error::Error;

use crate::data_structure::bounding_box::BoundingBox;

pub fn get_city_coordinates(city_name: &str) -> Result<(f64, f64), Box<dyn Error>> {
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
            return Ok((lat, lon));
        }
    }

    Err(format!("Cidade '{}' não encontrada", city_name).into())
}


pub fn get_all_cities_from_bounding_box(
    bbox: BoundingBox,
) -> Result<Vec<(String, f64, f64)>, Box<dyn Error>> {
    let client = Client::new();

    let query = format!(
        r#"[out:json];
        node
          ["place"~"city|town|village|hamlet"]
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

    Ok(cities)
}