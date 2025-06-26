use crate::data_structure::graph::GraphStructure;
use petgraph::graph::NodeIndex;

pub fn backtracking(graph: &GraphStructure) -> Option<Vec<NodeIndex>> {
    let root = graph.get_root()?;
    let objective = graph.get_objective()?;

    let path = recursive_backtracking(root, objective, graph);
    
    path
}

fn recursive_backtracking(
    current: NodeIndex,
    objective: NodeIndex,
    graph: &GraphStructure,
) -> Option<Vec<NodeIndex>> {

    
    let neighbors: Vec<NodeIndex> = graph.get_neighbors(current).collect();
    
    if current == objective {
        return Some(vec![current]);
    }

    for neighbor in neighbors {
        
        if let Some(mut path) = recursive_backtracking(neighbor, objective, graph) {
            path.insert(0, current);
            return Some(path);
        }
    }

    None
}