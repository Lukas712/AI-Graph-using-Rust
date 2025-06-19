mod functions;
use functions::calculate::calculate_level;

use functions::get_from_api::get_city_coordinates;

mod data_structure;
use data_structure::city::City;

use data_structure::bounding_box::BoundingBox;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::visit::NodeIndexable;
use reqwest::blocking::Client;
use serde_json::Value;
use std::collections::{ BTreeMap, HashMap };
use std::error::Error;
use std::hash::Hash;

use crate::functions::calculate::calculate_distance_value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let origin_city = "São Paulo";
    let destination_city = "Rio de Janeiro";

    let (lat_dest, lon_dest) = get_city_coordinates(destination_city)?;
    let (lat_orig, lon_orig) = get_city_coordinates(origin_city)?;

    let mut city_orig = City::new(origin_city.to_string(), lat_orig, lon_orig, lat_dest, lon_dest);

    let mut city_dest = City::new(
        destination_city.to_string(),
        lat_dest,
        lon_dest,
        lat_dest,
        lon_dest
    );

    let margin = 0.5;
    let bbox = BoundingBox::from_cities((lat_orig, lon_orig), (lat_dest, lon_dest), margin);
    println!("Bounding Box: {:?}", bbox);

    let min_bands = 3;
    let max_band_km = 50.0;
    let distance: f64 = city_orig.get_heuristic_value();
    let n_bands = (distance / max_band_km).ceil() as usize;
    let n_bands = n_bands.max(min_bands);
    let band_width = distance / (n_bands as f64);
    println!("Número de faixas: {}", n_bands);
    println!("Largura de cada faixa: {:.2} km", band_width);

    city_orig.set_level(
        calculate_level(city_orig.get_heuristic_value(), city_orig.get_heuristic_value(), n_bands)
    );

    city_dest.set_level(
        calculate_level(city_dest.get_heuristic_value(), city_orig.get_heuristic_value(), n_bands)
    );

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

    let mut level_map: BTreeMap<usize, Vec<City>> = BTreeMap::new();

    level_map.entry(city_orig.get_level()).or_insert_with(Vec::new).push(city_orig.clone());

    level_map.entry(city_dest.get_level()).or_insert_with(Vec::new).push(city_dest.clone());

    if let Some(elements) = res["elements"].as_array() {
        for el in elements {
            let lat = el["lat"].as_f64();
            let lon = el["lon"].as_f64();

            if let (Some(latitude), Some(longitude)) = (lat, lon) {
                if let Some(tags) = el["tags"].as_object() {
                    if let Some(name_value) = tags.get("name") {
                        let name = name_value.as_str().unwrap_or("Unknown").to_string();
                        if
                            latitude != lat_orig &&
                            longitude != lon_orig &&
                            latitude != lat_dest &&
                            longitude != lon_dest
                        {
                            let mut city = City::new(name, latitude, longitude, lat_dest, lon_dest);
                            if city.get_heuristic_value() < city_orig.get_heuristic_value() {
                                city.set_level(
                                    calculate_level(
                                        city.get_heuristic_value(),
                                        city_orig.get_heuristic_value(),
                                        n_bands
                                    )
                                );

                                level_map
                                    .entry(city.get_level())
                                    .or_insert_with(Vec::new)
                                    .push(city.clone());
                            }
                        }
                    }
                }
            }
        }
    }
    let mut graph: Graph<City, f32> = Graph::new();
    let mut index_map: BTreeMap<usize, Vec<NodeIndex>> = BTreeMap::new();
    let mut root_node_index: Option<NodeIndex> = None; // Variável para guardar a raiz

    for (&level, cities) in &level_map {
        for city in cities {
            let node_idx = graph.add_node(city.clone());

            // Guarda o índice da raiz (São Paulo)
            if city.get_name() == origin_city.to_string() {
                root_node_index = Some(node_idx);
            }

            index_map.entry(level).or_default().push(node_idx);
        }
    }

    for (&level, node_idxs) in &index_map {
        if let Some(next_idxs) = index_map.get(&(level + 1)) {
            for &u in node_idxs {
                let lat_u = graph[u].get_latitude();
                let lon_u = graph[u].get_longitude();
                for &v in next_idxs {
                    let lat_v = graph[v].get_latitude();
                    let lon_v = graph[v].get_longitude();
                    let dist = calculate_distance_value(lat_u, lon_u, lat_v, lon_v);
                    graph.add_edge(u, v, dist as f32);
                }
            }
        }
    }

    println!("\n\n=== RELAÇÃO COMPLETA DE NÓS E FILHOS ===");
    for node_idx in graph.node_indices() {
        let city = &graph[node_idx];
        println!(
            "\nNó: {} (nível {} coordenada: ({}, {})",
            city.get_name(),
            city.get_level(),
            city.get_latitude(),
            city.get_longitude()
        );

        let mut has_children = false;
        for neighbor in graph.neighbors(node_idx) {
            has_children = true;
            let neighbor_city = &graph[neighbor];
            if let Some(edge_idx) = graph.find_edge(node_idx, neighbor) {
                let edge_weight = graph[edge_idx];
                println!(
                    "  Filho: {} (nível {}, dist real: {:.2} km)",
                    neighbor_city.get_name(),
                    neighbor_city.get_level(),
                    edge_weight
                );
            }
        }

        if !has_children {
            println!("  [Nenhum filho]");
        }
    }

    Ok(())
}
