use crate::data_structure::{graph::GraphStructure, search_results::SearchResult};
use std::collections::HashMap;
use petgraph::graph::NodeIndex;
use std::time::Instant;
use crate::functions::path_distance::calculate_path_distance;


/// Realiza uma busca gulosa no grafo, priorizando o custo mínimo acumulado entre as cidades.
pub fn ordered_search(graph: &GraphStructure) -> Option<SearchResult> {
    let start_time = Instant::now();
    let root = graph.get_root()?;
    let objective = graph.get_objective()?;

    let mut expanded: i32 = 0;
    let mut visited: i32 = 0;
    let mut total_expanded: i32 = 0;
    let mut max_depth: i32 = 0;

    let mut g_scores = HashMap::new();
    g_scores.insert(root, 0.0);

    let mut came_from = HashMap::new();

    let mut open_list = Vec::new();
    open_list.push(State {
        node: root,
        cost: 0.0,
        depth: 1,
    });

    while !open_list.is_empty() {
        open_list.sort_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap());
        
        let current_state = open_list.remove(0);
        let current = current_state.node;
        let current_cost = current_state.cost;
        let current_depth = current_state.depth;

        visited += 1;
        if current_depth > max_depth {
            max_depth = current_depth;
        }

        if current == objective {
            let mut path = vec![current];
            let mut prev = current;
            while let Some(&parent) = came_from.get(&prev) {
                path.push(parent);
                prev = parent;
            }
            path.reverse();
            
            return Some(SearchResult {
                path_distance: calculate_path_distance(graph, path.clone()),
                visited,
                expanded,
                avg_branching_factor: if expanded > 0 { 
                    total_expanded as f32 / expanded as f32 
                } else { 
                    0.0 
                },
                depth: current_depth,
                execution_time: start_time.elapsed(),
                path: Some(path),
            });
        }
        let neighbors: Vec<NodeIndex> = graph.get_neighbors(current).collect();
        expanded += 1;
        total_expanded += neighbors.len() as i32;

        for neighbor in neighbors {
            let edge_cost = graph.get_edge_weight(current, neighbor);
            let new_cost = current_cost + edge_cost;

            if new_cost < *g_scores.get(&neighbor).unwrap_or(&f64::INFINITY) {
                came_from.insert(neighbor, current);
                g_scores.insert(neighbor, new_cost);
                
                open_list.push(State {
                    node: neighbor,
                    cost: new_cost,
                    depth: current_depth + 1,
                });
            }
        }
    }

    return Some(SearchResult {
        path_distance: 0.0,
        visited,
        expanded,
        avg_branching_factor: if expanded > 0 { 
            total_expanded as f32 / expanded as f32 
        } else { 
            0.0 
        },
        depth: max_depth,
        execution_time: start_time.elapsed(),
        path: None,
    })
}


////// Estrutura para armazenar o estado de um nó na busca ordenada.
/// 
/// Atributos:
/// - `node`: Índice do nó no grafo.
/// - `cost`: Custo acumulado até o nó.
/// - `depth`: Profundidade do nó na árvore de busca.
#[derive(PartialEq, PartialOrd)]
struct State {
    node: NodeIndex,
    cost: f64,
    depth: i32,
}

impl Eq for State {}