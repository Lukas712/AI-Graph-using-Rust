use crate::data_structure::graph::GraphStructure;
use petgraph::graph::NodeIndex;

pub fn backtracking(graph: &GraphStructure) -> Option<Vec<NodeIndex>> {
    let root = graph.get_root()?;
    let objective = graph.get_objective()?;
    
    println!("Iniciando busca backtracking...");
    println!("Raiz: {:?}", root);
    println!("Objetivo: {:?}", objective);
    
    let path = recursive_backtracking(root, objective, graph);
    
    match &path {
        Some(p) => println!("Caminho encontrado: {:?}", p),
        None => println!("Nenhum caminho encontrado"),
    }
    
    path
}

fn recursive_backtracking(
    current: NodeIndex,
    objective: NodeIndex,
    graph: &GraphStructure,
) -> Option<Vec<NodeIndex>> {
    // Imprime o nó atual e seus vizinhos
    println!("\nVisitando nó: {:?}", graph.get_city(current).unwrap().get_name());
    
    let neighbors: Vec<NodeIndex> = graph.get_neighbors(current).collect();
    println!("Vizinhos de {:?}: {:?}", graph.get_city(current).unwrap().get_name(), neighbors);
    
    if current == objective {
        println!("--> Encontrou o objetivo!");
        return Some(vec![current]);
    }

    for neighbor in neighbors {
        println!("  Explorando vizinho {:?} de {:?}", neighbor, current);
        
        if let Some(mut path) = recursive_backtracking(neighbor, objective, graph) {
            path.insert(0, current);
            return Some(path);
        } else {
            println!("  Nenhum caminho encontrado através de {:?}", graph.get_city(neighbor).unwrap().get_name());
        }
    }

    println!("Nenhum caminho válido a partir de {:?}", current);
    None
}