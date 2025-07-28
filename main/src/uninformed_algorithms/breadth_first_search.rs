use crate::data_structure::graph::GraphStructure;
use petgraph::{graph::NodeIndex, visit};
use std::collections::{VecDeque, HashMap};
use std::time::Instant;
use crate::data_structure::search_results::SearchResult;
use crate::functions::path_distance::calculate_path_distance;

pub fn breadth_first_search(graph: &GraphStructure) -> Option<SearchResult> {
    let start_time = Instant::now();
    let root = graph.get_root()?;
    let objective = graph.get_objective()?;

    let mut expanded: i32 = 0;
    let mut visited: i32 = 0;
    let mut total_expanded: i32 = 0;
    let mut max_depth: i32 = 0;

    let mut queue = VecDeque::new();
    queue.push_back((root, 1));

    let mut parent_map = HashMap::new();
    parent_map.insert(root, root);

    while let Some((current, current_deep)) = queue.pop_front() {
        visited += 1;

        if current == objective {
            let path = reconstruct_path(&parent_map, current, root);

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
        if current_deep > max_depth {
            max_depth = current_deep;
        }

        let neighbors: Vec<NodeIndex> = graph.get_neighbors(current).collect();
        expanded += 1;
        total_expanded += neighbors.len() as i32;

        for neighbor in neighbors {
            if !parent_map.contains_key(&neighbor) {
                parent_map.insert(neighbor, current);
                queue.push_back((neighbor, current_deep + 1));
            }
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

fn reconstruct_path(
    parent_map: &HashMap<NodeIndex, NodeIndex>,
    mut current: NodeIndex,
    root: NodeIndex
) -> Vec<NodeIndex> {
    let mut path = Vec::new();

    while current != root {
        path.push(current);
        current = parent_map[&current];
    }

    path.push(root);

    path.reverse();
    path
}

