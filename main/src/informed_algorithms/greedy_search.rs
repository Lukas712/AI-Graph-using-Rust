use crate::data_structure::graph::GraphStructure;
use petgraph::graph::NodeIndex;
use std::time::Instant;
use crate::data_structure::search_results::SearchResult;
use crate::functions::path_distance::calculate_path_distance;

pub fn greedy_search(graph: &GraphStructure) -> Option<SearchResult> {
    let start_time = Instant::now();
    let root = graph.get_root()?;
    let objective = graph.get_objective()?;

    let mut expanded: i32 = 0;
    let mut visited: i32 = 0;
    let mut total_expanded: i32 = 0;
    let mut max_depth: i32 = 0;

    let mut path_stack = vec![root];
    let mut fully_explored = Vec::new();

    while let Some(&current) = path_stack.last() {
        visited += 1;
        let current_deep = path_stack.len() as i32;
        
        if current_deep > max_depth {
            max_depth = current_deep;
        }

        if current == objective {
            return Some(SearchResult {
                path_distance: calculate_path_distance(graph, path_stack.clone()),
                visited,
                expanded,
                avg_branching_factor: if expanded > 0 { total_expanded as f32 / expanded as f32 } else { 0.0 },
                depth: current_deep,
                execution_time: start_time.elapsed(),
                path: Some(path_stack),
            });
        }

        let neighbors = graph.get_neighbors(current).collect::<Vec<_>>();
        total_expanded += neighbors.len() as i32;
        expanded += 1;

        let mut best_neighbor = None;
        let mut best_heuristic = f64::MAX;

        for &neighbor in &neighbors {
            if !fully_explored.contains(&neighbor) {
                if let Some(city) = graph.get_city(neighbor) {
                    let heuristic = city.get_heuristic_value();
                    if heuristic < best_heuristic {
                        best_heuristic = heuristic;
                        best_neighbor = Some(neighbor);
                    }
                }
            }
        }

        if let Some(next_node) = best_neighbor {
            path_stack.push(next_node);
        } else {
            fully_explored.push(current);
            path_stack.pop();
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