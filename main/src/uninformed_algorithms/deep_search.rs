use crate::data_structure::graph::GraphStructure;
use petgraph::graph::NodeIndex;
use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};

pub fn depth_limited_search(
    graph: &GraphStructure,
    depth_limit: usize,
) -> Option<Vec<NodeIndex>> {
    let root = graph.get_root()?;
    let objective = graph.get_objective()?;

    let neighbors: Vec<NodeIndex> = graph.get_neighbors(root).collect();
    
    neighbors.into_par_iter().find_map_any(|neighbor| {
        let mut visited = HashSet::new();
        visited.insert(root);
        visited.insert(neighbor);
        
        let mut stack = VecDeque::new();
        stack.push_back((neighbor, 1, vec![root, neighbor]));
        
        dls_task(graph, objective, &mut stack, &mut visited, depth_limit)
    })
}

fn dls_task(
    graph: &GraphStructure,
    objective: NodeIndex,
    stack: &mut VecDeque<(NodeIndex, usize, Vec<NodeIndex>)>,
    visited: &mut HashSet<NodeIndex>,
    depth_limit: usize,
) -> Option<Vec<NodeIndex>> {
    while let Some((current, depth, path)) = stack.pop_back() {
        if current == objective {
            return Some(path);
        }

        if depth >= depth_limit {
            continue;
        }

        let neighbors: Vec<NodeIndex> = graph.get_neighbors(current)
            .filter(|n| !visited.contains(n))
            .collect();
        
        for neighbor in neighbors {
            visited.insert(neighbor);
            let mut new_path = path.clone();
            new_path.push(neighbor);
            stack.push_back((neighbor, depth + 1, new_path));
        }
    }
    None
}