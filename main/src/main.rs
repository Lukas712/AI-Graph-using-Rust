mod data_structure;
use data_structure::map::Map;

mod functions;

mod uninformed_algorithms;
use uninformed_algorithms::backtracking::backtracking;
use uninformed_algorithms::breadth_first_search::breadth_first_search;
use uninformed_algorithms::ordered_search::ordered_search;
use uninformed_algorithms::deep_search::depth_search;

mod informed_algorithms;
use informed_algorithms::greedy_search::greedy_search;
use informed_algorithms::a_star::a_star_search;
use informed_algorithms::ida_star::ida_star_search;

use crate::data_structure::city;
use crate::data_structure::graph::GraphStructure;
use crate::data_structure::search_results::SearchResult;
use crate::functions::path_distance::calculate_path_distance;
use crate::functions::results::{print_no_result, print_path};

use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("\n--- Busca de Caminho entre Cidades ---");
        println!("Digite 'sair' a qualquer momento para encerrar o programa.");
        println!("Digite o nome das cidades se atentando à acentuação e ortografia.");
        println!();
        let origin_city = read_input("Digite a cidade de origem, (ou 'sair' para encerrar): ");
        if origin_city.trim().to_lowercase() == "sair" {
            break;
        }

        let destination_city = read_input("Digite a cidade de destino (ou 'sair' para encerrar): ");
        if destination_city.trim().to_lowercase() == "sair" {
            break;
        }

        let map = match Map::new(origin_city.trim().to_string(), destination_city.trim().to_string()) {
            Ok(map) => map,
            Err(e) => {
                println!("Erro ao criar mapa: {}", e);
                continue;
            }
        };

        loop {
            println!("\nSelecione o algoritmo de busca:");
            println!("1. Backtracking");
            println!("2. Breadth-first search");
            println!("3. Ordered search");
            println!("4. Depth search");
            println!("5. Greedy search");
            println!("6. A* search");
            println!("7. IDA* search");
            println!("8. Todos os algoritmos");
            println!("9. Mostrar mapa");
            println!("10. Mudar cidades");
            println!("0. Sair");

            let choice = read_input("Opção: ");

            match choice.as_str() {
                "1" => run_algorithm(&map, "Backtracking", backtracking),
                "2" => run_algorithm(&map, "Breadth-first search", breadth_first_search),
                "3" => run_algorithm(&map, "Ordered search", ordered_search),
                "4" => run_algorithm(&map, "Depth search", depth_search),
                "5" => run_algorithm(&map, "Greedy search", greedy_search),
                "6" => run_algorithm(&map, "A* search", a_star_search),
                "7" => run_algorithm(&map, "IDA* search", ida_star_search),
                "8" => {
                    run_algorithm(&map, "Backtracking", backtracking);
                    run_algorithm(&map, "Breadth-first search", breadth_first_search);
                    run_algorithm(&map, "Ordered search", ordered_search);
                    run_algorithm(&map, "Depth search", depth_search);
                    run_algorithm(&map, "Greedy search", greedy_search);
                    run_algorithm(&map, "A* search", a_star_search);
                    run_algorithm(&map, "IDA* search", ida_star_search);
                },
                "9" => {
                    println!("\n--- Mapa ---");
                    map.print_graph_by_levels();
                },
                "10" => break,
                "0" => return Ok(()),
                _ => println!("Opção inválida!"),
            }
        }
    }

    Ok(())
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().to_string()
}

fn run_algorithm<F>(map: &Map, name: &str, algorithm: F)
where
    F: Fn(&GraphStructure) -> Option<SearchResult>,
{
    println!("\n--- {} ---", name);
    algorithm(map.get_graph())
        .map(|search_result| {
            print_path(search_result, &map);
        })
        .unwrap_or_else(|| {
            print_no_result();
        });
}