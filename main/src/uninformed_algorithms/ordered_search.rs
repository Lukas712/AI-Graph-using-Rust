use crate::data_structure::graph::GraphStructure;
use std::collections::{BinaryHeap, HashMap};
use petgraph::graph::NodeIndex;

pub fn ordered_search(graph: &GraphStructure) -> Option<Vec<NodeIndex>> {
    let root = graph.get_root()?;
    let objective = graph.get_objective()?;

    let mut g_scores = HashMap::new();
    g_scores.insert(root, 0.0);

    let mut came_from = HashMap::new();

    let mut open_set = BinaryHeap::new();
    open_set.push(State {
        node: root,
        cost: 0.0,
    });

    while let Some(State { node: current, cost: current_cost }) = open_set.pop() {
        if current == objective {
            let mut path = vec![current];
            let mut prev = current;
            while let Some(&parent) = came_from.get(&prev) {
                path.push(parent);
                prev = parent;
            }
            path.reverse();
            return Some(path);
        }

        for neighbor in graph.get_neighbors(current) {
            let new_cost = current_cost + graph.get_edge_weight(current, neighbor);

            if new_cost < *g_scores.get(&neighbor).unwrap_or(&f64::INFINITY) {
                came_from.insert(neighbor, current);
                g_scores.insert(neighbor, new_cost);
                open_set.push(State {
                    node: neighbor,
                    cost: new_cost,
                });
            }
        }
    }

    None
}

#[derive(PartialEq, PartialOrd)]
struct State {
    node: NodeIndex,
    cost: f64,
}

impl Eq for State {}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.partial_cmp(&self.cost).unwrap()
    }
}