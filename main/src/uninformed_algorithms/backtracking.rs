use crate::data_structure::graph::GraphStructure;
use petgraph::graph::NodeIndex;
use std::{collections::VecDeque, time::Instant};
use crate::data_structure::search_results::SearchResult;
use crate::functions::path_distance::calculate_path_distance;

/// Realiza uma busca por retrocesso (backtracking) no grafo, explorando todos os caminhos possíveis, mesmo que já tenham sido visitados.
pub fn backtracking(graph: &GraphStructure) -> Option<SearchResult> {
    let start_time = Instant::now();
    let root = graph.get_root()?;
    let objective = graph.get_objective()?;

    let mut expanded: i32 = 0;
    let mut visited: i32 = 0;
    let mut total_expanded: i32 = 0;
    let mut max_depth: i32 = 0;

    let mut stack = VecDeque::new();
    stack.push_back((root, vec![root], 1));

    while let Some((current, path, current_deep)) = stack.pop_back() {
        visited += 1;
        
        if current_deep > max_depth {
            max_depth = current_deep;
        }

        if current == objective {
            return Some(SearchResult {
                path_distance: calculate_path_distance(graph, path.clone()),
                visited,
                expanded,
                avg_branching_factor: if expanded > 0 { total_expanded as f32 / expanded as f32 } else { 0.0 },
                depth: current_deep,
                execution_time: start_time.elapsed(),
                path: Some(path),
            });
        }

        let neighbors: Vec<NodeIndex> = graph.get_neighbors(current).collect();
        total_expanded += neighbors.len() as i32;
        expanded += 1;
        
        for neighbor in neighbors.into_iter().rev() {
            let mut new_path = path.clone();
            new_path.push(neighbor);
            stack.push_back((neighbor, new_path, current_deep + 1));
        }
    }

    Some(SearchResult {
        path_distance: 0.0,
        visited,
        expanded,
        avg_branching_factor: if expanded > 0 { total_expanded as f32 / expanded as f32 } else { 0.0 },
        depth: 0,
        execution_time: start_time.elapsed(),
        path: None,
    })
}