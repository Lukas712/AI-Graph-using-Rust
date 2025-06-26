use crate::data_structure::graph::GraphStructure;
use std::collections::{BinaryHeap, HashMap};
use petgraph::graph::NodeIndex;

pub fn a_star_search(graph: &GraphStructure) -> Option<Vec<NodeIndex>> {
    let root = graph.get_root()?;
    let objective = graph.get_objective()?;

    let mut g_scores = HashMap::new();
    g_scores.insert(root, 0.0);

    let mut came_from = HashMap::new();

    let mut open_set = BinaryHeap::new();
    open_set.push(State {
        node: root,
        f_score: 0.0 + graph.get_city(root).unwrap().get_heuristic_value(),
    });

    while let Some(State { node: current, f_score: _ }) = open_set.pop() {
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
            let tentative_g_score = g_scores[&current] + graph.get_edge_weight(current, neighbor);

            if tentative_g_score < *g_scores.get(&neighbor).unwrap_or(&f64::INFINITY) {
                came_from.insert(neighbor, current);
                g_scores.insert(neighbor, tentative_g_score);

                let f_score = tentative_g_score + graph.get_city(neighbor).unwrap().get_heuristic_value();
                
                open_set.push(State {
                    node: neighbor,
                    f_score,
                });
            }
        }
    }

    None
}

#[derive(PartialEq, PartialOrd)]
struct State {
    node: NodeIndex,
    f_score: f64,
}

impl Eq for State {}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_score.partial_cmp(&self.f_score).unwrap()
    }
}