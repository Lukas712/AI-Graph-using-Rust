use crate::data_structure::graph::GraphStructure;
use crate::data_structure::search_results::SearchResult;
use crate::functions::path_distance::calculate_path_distance;
use petgraph::graph::NodeIndex;
use std::collections::HashSet;
use std::time::Instant;


/// Realiza uma busca A* iterativa (IDA*) no grafo, utilizando heurísticas para otimizar a busca.
pub fn ida_star_search(graph: &GraphStructure) -> Option<SearchResult> {
    let start_time = Instant::now();
    let root = graph.get_root()?;
    let objective = graph.get_objective()?;

    let mut visited = 0;
    let mut expanded = 0;
    let mut total_expanded = 0;
    let mut max_depth = 0;

    let mut threshold = graph.get_city(root)?.get_heuristic_value();
    let mut next_threshold = f64::INFINITY;

    loop {
        let mut stack = Vec::new();
        let mut path = Vec::new();
        let mut path_set = HashSet::new();
        let mut g_values = Vec::new();

        path.push(root);
        path_set.insert(root);
        g_values.push(0.0);
        stack.push((root, false, graph.get_neighbors(root).collect::<Vec<_>>()));

        visited += 1;

        while let Some((current, is_backtracking, neighbors)) = stack.last_mut() {
            let current_g = *g_values.last().unwrap();
            let current_f = current_g + graph.get_city(*current).unwrap().get_heuristic_value();

            let current_depth = path.len() as i32;
            if current_depth > max_depth {
                max_depth = current_depth;
            }

            if current_f > threshold {
                if current_f < next_threshold {
                    next_threshold = current_f;
                }
                stack.pop();
                path_set.remove(&path.pop().unwrap());
                g_values.pop();
                continue;
            }

            if *current == objective {
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

            if *is_backtracking || neighbors.is_empty() {
                stack.pop();
                path_set.remove(&path.pop().unwrap());
                g_values.pop();
                continue;
            }

            let neighbor = neighbors.pop().unwrap();
            *is_backtracking = true;

            if path_set.contains(&neighbor) {
                continue;
            }

            let new_g = current_g + graph.get_edge_weight(*current, neighbor);
            path.push(neighbor);
            path_set.insert(neighbor);
            g_values.push(new_g);
            let neighbor_neighbors = graph.get_neighbors(neighbor).collect::<Vec<_>>();
            total_expanded += neighbor_neighbors.len() as i32;
            expanded += 1;
            visited += 1;
            stack.push((neighbor, false, neighbor_neighbors));
        }

        if next_threshold == f64::INFINITY {
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
            });
        }

        threshold = next_threshold;
        next_threshold = f64::INFINITY;
    }
}