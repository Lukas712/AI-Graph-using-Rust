use crate::data_structure::{map::Map, search_results::SearchResult};

fn print_results(search_result: SearchResult) {
    println!("\nEstatísticas da busca:");
    println!("- Distância do caminho: {:.2} KM", search_result.path_distance);
    println!("- Nós visitados: {}", search_result.visited);
    println!("- Nós expandidos: {}", search_result.expanded);
    println!("- Profundidade máxima: {}", search_result.depth);
    println!("- Fator de ramificação médio: {:.2}", search_result.avg_branching_factor);
    println!("- Tempo de execução: {:.4} segundos", search_result.execution_time.as_secs_f64());
    println!();
}

pub fn print_path(search_result: SearchResult, map: &Map) {
    
    if let Some(ref path) = search_result.path {
        println!("Caminho encontrado:");
        for city in path {
            println!("Cidade: {}, Heurística: {}, Latitude: {}, Longitude: {}", 
            map.get_graph().get_city(*city).unwrap().get_name(), 
            map.get_graph().get_city(*city).unwrap().get_heuristic_value(), 
            map.get_graph().get_city(*city).unwrap().get_latitude(), 
            map.get_graph().get_city(*city).unwrap().get_longitude());
        }      
    } else {
        println!("Nenhum caminho encontrado.");
        println!();
    }

    print_results(search_result);
    println!();
}

pub fn print_no_result(){
    println!("Nenhum resultado retornado (grafo inválido).");
    println!();
}