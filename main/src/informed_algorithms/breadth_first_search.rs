use crate::data_structure::graph::GraphStructure;
use petgraph::graph::NodeIndex;
use std::collections::{VecDeque, HashMap};

pub fn breadth_first_search(graph: &GraphStructure) -> Option<Vec<NodeIndex>> {
    let root = graph.get_root()?;
    let objective = graph.get_objective()?;

    println!("Iniciando busca em largura...");
    println!("Raiz: {:?}", graph.get_city(root).unwrap().get_name());
    println!("Objetivo: {:?}", graph.get_city(objective).unwrap().get_name());

    let mut queue = VecDeque::new();
    queue.push_back(root);

    let mut parent_map = HashMap::new();
    parent_map.insert(root, root);

    while let Some(current) = queue.pop_front() {
        println!("\nVisitando nó: {:?}", graph.get_city(current).unwrap().get_name());

        let neighbors: Vec<NodeIndex> = graph.get_neighbors(current).collect();
        println!("Vizinhos de {:?}: {:?}", 
            graph.get_city(current).unwrap().get_name(),
            neighbors.iter().map(|&n| graph.get_city(n).unwrap().get_name()).collect::<Vec<_>>());

        for neighbor in neighbors {
            if !parent_map.contains_key(&neighbor) {
                println!("  Adicionando vizinho {:?} à fila", 
                    graph.get_city(neighbor).unwrap().get_name());
                
                parent_map.insert(neighbor, current);
                queue.push_back(neighbor);
            }
        }

        if current == objective {
            println!("--> Encontrou o objetivo!");
            return Some(reconstruct_path(&parent_map, current));
        }
    }

    println!("Nenhum caminho encontrado");
    None
}

fn reconstruct_path(parent_map: &HashMap<NodeIndex, NodeIndex>, mut current: NodeIndex) -> Vec<NodeIndex> {
    let mut path = Vec::new();
    let root = *parent_map.get(&current).unwrap();  // Assume que o mapa está correto

    // Reconstrói do objetivo até a raiz
    while current != root {
        path.push(current);
        current = parent_map[&current];
    }
    path.push(root);
    path.reverse();  // Inverte para ordem raiz->objetivo

    path
}