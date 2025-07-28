use crate::data_structure::graph::GraphStructure;
use petgraph::graph::NodeIndex;
use std::time::Instant;
use crate::data_structure::search_results::SearchResult;
use crate::functions::path_distance::calculate_path_distance;
use std::collections::{HashSet, VecDeque};


/// Realiza uma busca em profundidade no grafo, explorando os nós mais novos do grafo.
pub fn depth_search(
    graph: &GraphStructure,
) -> Option<SearchResult> {
    let start_time = Instant::now();
    let root = graph.get_root()?;
    let objective = graph.get_objective()?;

    let mut visited: i32 = 0;
    let mut expanded: i32 = 0;
    let mut total_expanded: i32 = 0;
    let mut max_depth: i32 = 0;

    let mut visited_nodes = HashSet::new();
    visited_nodes.insert(root);
    
    let mut stack = VecDeque::new();
    stack.push_back((root, 0, vec![root]));

    while let Some((current, depth, path)) = stack.pop_back() {
        visited += 1;
        
        let current_depth = depth as i32;
        if current_depth > max_depth {
            max_depth = current_depth;
        }

        if current == objective {
            return Some(SearchResult {
                path_distance: calculate_path_distance(graph, path.clone()),
                visited,
                expanded,
                avg_branching_factor: if expanded > 0 { total_expanded as f32 / expanded as f32 } else { 0.0 },
                depth: current_depth,
                execution_time: start_time.elapsed(),
                path: Some(path),
            });
        }

        let neighbors: Vec<NodeIndex> = graph.get_neighbors(current)
            .filter(|n| !visited_nodes.contains(n))
            .collect();
        
        total_expanded += neighbors.len() as i32;
        expanded += 1;

        for neighbor in neighbors.into_iter().rev() {
            visited_nodes.insert(neighbor);
            let mut new_path = path.clone();
            new_path.push(neighbor);
            stack.push_back((neighbor, depth + 1, new_path));
        }
    }

    Some(SearchResult {
        path_distance: 0.0,
        visited,
        expanded,
        avg_branching_factor: if expanded > 0 { total_expanded as f32 / expanded as f32 } else { 0.0 },
        depth: max_depth,
        execution_time: start_time.elapsed(),
        path: None,
    })
}