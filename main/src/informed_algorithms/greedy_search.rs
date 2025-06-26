use crate::data_structure::graph::GraphStructure;
use petgraph::graph::NodeIndex;

pub fn greedy_search(graph: &GraphStructure) -> Option<Vec<NodeIndex>> {
    let root = graph.get_root()?;
    let objective = graph.get_objective()?;

    let mut path_stack = vec![root];
    let mut fully_explored = Vec::new();

    while let Some(&current) = path_stack.last() {

        if current == objective {
            return Some(path_stack.clone());
        }

        if let Some(next_node) = find_best_heuristic_neighbor(graph, current, &fully_explored) {
            path_stack.push(next_node);
        } else {
            fully_explored.push(current);
            path_stack.pop();
        }
    }

    None
}

fn find_best_heuristic_neighbor(
    graph: &GraphStructure,
    current: NodeIndex,
    fully_explored: &[NodeIndex],
) -> Option<NodeIndex> {
    let neighbors = graph.get_neighbors(current).collect::<Vec<_>>();
    
    let unexplored: Vec<_> = neighbors
        .iter()
        .filter(|&&n| !fully_explored.contains(&n))
        .collect();

    if unexplored.is_empty() {
        return None;
    }

    let mut best_neighbor = **unexplored.first().unwrap();
    let mut best_heuristic = graph.get_city(best_neighbor).unwrap().get_heuristic_value();

    for &&neighbor in &unexplored {
        let heuristic = graph.get_city(neighbor).unwrap().get_heuristic_value();
        if heuristic < best_heuristic {
            best_heuristic = heuristic;
            best_neighbor = neighbor;
        }
    }

    Some(best_neighbor)
}