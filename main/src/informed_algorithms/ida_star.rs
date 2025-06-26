use crate::data_structure::graph::GraphStructure;
use petgraph::graph::NodeIndex;
use std::collections::HashSet;

pub fn ida_star_search(graph: &GraphStructure) -> Option<Vec<NodeIndex>> {
    let root = graph.get_root()?;
    let objective = graph.get_objective()?;

    let mut threshold = graph.get_city(root).unwrap().get_heuristic_value();
    
    let mut path = Vec::new();
    let mut path_set = HashSet::new();

    path.push(root);
    path_set.insert(root);

    loop {
        let (result, new_threshold) = depth_first(
            root, 
            0.0, 
            threshold, 
            graph, 
            objective, 
            &mut path, 
            &mut path_set
        );
        
        if let Some(path_result) = result {
            return Some(path_result);
        }

        if new_threshold == f64::INFINITY {
            return None;
        }

        threshold = new_threshold;
    }
}

fn depth_first(
    current: NodeIndex,
    current_g: f64,
    threshold: f64,
    graph: &GraphStructure,
    objective: NodeIndex,
    path: &mut Vec<NodeIndex>,
    path_set: &mut HashSet<NodeIndex>,
) -> (Option<Vec<NodeIndex>>, f64) {
    let current_f = current_g + graph.get_city(current).unwrap().get_heuristic_value();

    if current_f > threshold {
        return (None, current_f);
    }

    if current == objective {
        return (Some(path.clone()), current_f);
    }
    
    let mut next_threshold = f64::INFINITY;

    for neighbor in graph.get_neighbors(current) {
        if path_set.contains(&neighbor) {
            continue;
        }
        
        path.push(neighbor);
        path_set.insert(neighbor);

        let new_g = current_g + graph.get_edge_weight(current, neighbor);

        let (result, new_t) = depth_first(
            neighbor, 
            new_g, 
            threshold, 
            graph, 
            objective, 
            path, 
            path_set
        );

        if result.is_some() {
            return (result, threshold);
        }

        if new_t < next_threshold {
            next_threshold = new_t;
        }

        path.pop();
        path_set.remove(&neighbor);
    }
    
    (None, next_threshold)
}