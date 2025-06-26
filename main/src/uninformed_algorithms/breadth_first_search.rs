use crate::data_structure::graph::GraphStructure;
use petgraph::graph::NodeIndex;
use std::collections::{VecDeque, HashMap};

pub fn breadth_first_search(graph: &GraphStructure) -> Option<Vec<NodeIndex>> {
    let root = graph.get_root()?;
    let objective = graph.get_objective()?;

    let mut queue = VecDeque::new();
    queue.push_back(root);

    let mut parent_map = HashMap::new();
    parent_map.insert(root, root);

    while let Some(current) = queue.pop_front() {

        let neighbors: Vec<NodeIndex> = graph.get_neighbors(current).collect();

        for neighbor in neighbors {
            if !parent_map.contains_key(&neighbor) {
                parent_map.insert(neighbor, current);
                queue.push_back(neighbor);
            }
        }

        if current == objective {
            return Some(reconstruct_path(&parent_map, current, root));
        }
    }
    
    None
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

