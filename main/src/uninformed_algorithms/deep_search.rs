use crate::data_structure::graph::GraphStructure;
use petgraph::graph::NodeIndex;
use std::collections::{HashSet, VecDeque};

pub fn depth_limited_search(
    graph: &GraphStructure,
    depth_limit: usize,
) -> Option<Vec<NodeIndex>> {
    let root = graph.get_root()?;
    let objective = graph.get_objective()?;

    let mut stack = VecDeque::new();
    stack.push_back((root, 0, vec![root]));
    
    let mut visited = HashSet::new();
    visited.insert(root);

    while let Some((current, depth, path)) = stack.pop_back() {
        if current == objective {
            return Some(path);
        }

        if depth >= depth_limit {
            continue;
        }

        for neighbor in graph.get_neighbors(current) {
            // Evita revisitar nós no mesmo caminho
            if visited.contains(&neighbor) {
                continue;
            }

            visited.insert(neighbor);
            let mut new_path = path.clone();
            new_path.push(neighbor);
            
            stack.push_back((neighbor, depth + 1, new_path));
        }
    }

    None
}