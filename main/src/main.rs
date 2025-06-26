mod data_structure;
use data_structure::map::Map;

mod functions;

mod uninformed_algorithms;
use uninformed_algorithms::backtracking::backtracking;
use uninformed_algorithms::breadth_first_search::breadth_first_search;
use uninformed_algorithms::ordered_search::ordered_search;
use uninformed_algorithms::deep_search::depth_limited_search;

mod informed_algorithms;
use informed_algorithms::greedy_search::greedy_search;
use informed_algorithms::a_star::a_star_search;
use informed_algorithms::ida_star::ida_star_search;

use crate::data_structure::city;
use crate::functions::path_distance::calculate_path_distance;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let origin_city = "Eugenópolis";
    let destination_city = "Juiz de Fora";

    let map = Map::new(
        origin_city.to_string(),
        destination_city.to_string(),
    )?;

    // map.print_graph_by_levels();

    // backtracking(map.get_graph().clone())
    //     .map(|path| {
    //         println!("Caminho encontrado:");
    //         for city in &path {
    //             println!("Cidade: {}, Heurística: {}, Latitude: {}, Longitude: {}", map.get_graph().get_city(*city).unwrap().get_name(), map.get_graph().get_city(*city).unwrap().get_heuristic_value(), map.get_graph().get_city(*city).unwrap().get_latitude(), map.get_graph().get_city(*city).unwrap().get_longitude());
    //         }
    //         calculate_path_distance(map.get_graph(), path);
    //     })
    //     .unwrap_or_else(|| {
    //         println!("Nenhum caminho encontrado.");
    //     });
    
    // breadth_first_search(map.get_graph().clone())
    //     .map(|path| {
    //         println!("Caminho encontrado:");
    //         for city in &path {
    //             println!("Cidade: {}, Heurística: {}, Latitude: {}, Longitude: {}", map.get_graph().get_city(*city).unwrap().get_name(), map.get_graph().get_city(*city).unwrap().get_heuristic_value(), map.get_graph().get_city(*city).unwrap().get_latitude(), map.get_graph().get_city(*city).unwrap().get_longitude());
    //         }
    //         calculate_path_distance(map.get_graph(), path);
    //     })
    //     .unwrap_or_else(|| {
    //         println!("Nenhum caminho encontrado.");
    //     });
    
    ordered_search(map.get_graph().clone())
        .map(|path| {
            println!("Caminho encontrado:");
            for city in &path {
                println!("Cidade: {}, Heurística: {}, Latitude: {}, Longitude: {}", map.get_graph().get_city(*city).unwrap().get_name(), map.get_graph().get_city(*city).unwrap().get_heuristic_value(), map.get_graph().get_city(*city).unwrap().get_latitude(), map.get_graph().get_city(*city).unwrap().get_longitude());
            }
            calculate_path_distance(map.get_graph(), path);
        })
        .unwrap_or_else(|| {
            println!("Nenhum caminho encontrado.");
        });

    // greedy_search(map.get_graph().clone())
    //     .map(|path| {
    //         println!("Caminho encontrado:");
    //         for city in &path {
    //             println!("Cidade: {}, Heurística: {}, Latitude: {}, Longitude: {}", map.get_graph().get_city(*city).unwrap().get_name(), map.get_graph().get_city(*city).unwrap().get_heuristic_value(), map.get_graph().get_city(*city).unwrap().get_latitude(), map.get_graph().get_city(*city).unwrap().get_longitude());
    //         }
    //         calculate_path_distance(map.get_graph(), path);
    //     })
    //     .unwrap_or_else(|| {
    //         println!("Nenhum caminho encontrado.");
    //     });

    // a_star_search(map.get_graph().clone())
    //     .map(|path| {
    //         println!("Caminho encontrado:");
    //         for city in &path {
    //             println!("Cidade: {}, Heurística: {}, Latitude: {}, Longitude: {}", map.get_graph().get_city(*city).unwrap().get_name(), map.get_graph().get_city(*city).unwrap().get_heuristic_value(), map.get_graph().get_city(*city).unwrap().get_latitude(), map.get_graph().get_city(*city).unwrap().get_longitude());
    //         }
    //         calculate_path_distance(map.get_graph(), path);
    //     })
    //     .unwrap_or_else(|| {
    //         println!("Nenhum caminho encontrado.");
    //     });

    // ida_star_search(map.get_graph().clone())
    //     .map(|path| {
    //         println!("Caminho encontrado:");
    //         for city in &path {
    //             println!("Cidade: {}, Heurística: {}, Latitude: {}, Longitude: {}", map.get_graph().get_city(*city).unwrap().get_name(), map.get_graph().get_city(*city).unwrap().get_heuristic_value(), map.get_graph().get_city(*city).unwrap().get_latitude(), map.get_graph().get_city(*city).unwrap().get_longitude());
    //         }
    //         calculate_path_distance(map.get_graph(), path);
    //     })
    //     .unwrap_or_else(|| {
    //         println!("Nenhum caminho encontrado.");
    //     });

    depth_limited_search(map.get_graph().clone(), map.get_number_of_levels())
        .map(|path| {
            println!("Caminho encontrado:");
            for city in &path {
                println!("Cidade: {}, Heurística: {}, Latitude: {}, Longitude: {}", map.get_graph().get_city(*city).unwrap().get_name(), map.get_graph().get_city(*city).unwrap().get_heuristic_value(), map.get_graph().get_city(*city).unwrap().get_latitude(), map.get_graph().get_city(*city).unwrap().get_longitude());
            }
            calculate_path_distance(map.get_graph(), path);
        })
        .unwrap_or_else(|| {
            println!("Nenhum caminho encontrado.");
        });

    Ok(())
}
