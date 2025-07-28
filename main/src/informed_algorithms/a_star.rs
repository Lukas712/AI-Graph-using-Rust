use crate::data_structure::graph::GraphStructure;
use std::collections::{BinaryHeap, HashMap};
use petgraph::graph::NodeIndex;
use std::time::Instant;
use crate::data_structure::search_results::SearchResult;
use crate::functions::path_distance::calculate_path_distance;

/// Realiza uma busca A* no grafo, utilizando o acúmulo de distâncias e o valor da heurística como base.
pub fn a_star_search(graph: &GraphStructure) -> Option<SearchResult> {
    let start_time = Instant::now();
    let root = graph.get_root()?;
    let objective = graph.get_objective()?;

    let mut visited: i32 = 0;
    let mut expanded: i32 = 0;
    let mut total_expanded: i32 = 0;
    let mut max_depth: i32 = 0;

    let mut g_scores = HashMap::new();
    g_scores.insert(root, 0.0);

    let mut came_from = HashMap::new();
    let mut open_set = BinaryHeap::new();
    
    open_set.push(State {
        node: root,
        f_score: 0.0 + graph.get_city(root).unwrap().get_heuristic_value(),
    });

    while let Some(State { node: current, f_score: _ }) = open_set.pop() {
        visited += 1;
        let current_deep = came_from.len() as i32 + 1;
        
        if current_deep > max_depth {
            max_depth = current_deep;
        }

        if current == objective {
            let mut path = vec![current];
            let mut prev = current;
            while let Some(&parent) = came_from.get(&prev) {
                path.push(parent);
                prev = parent;
            }
            path.reverse();

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

        let neighbors = graph.get_neighbors(current).collect::<Vec<_>>();
        total_expanded += neighbors.len() as i32;
        expanded += 1;

        for neighbor in neighbors {
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

    Some(SearchResult {
        path_distance: 0.0,
        visited,
        expanded,
        avg_branching_factor: if expanded > 0 { total_expanded as f32 / expanded as f32 } else { 0.0 },
        depth: max_depth,
        execution_time: start_time.elapsed(),
        path: None,
    })
}


/// Estrutura para armazenar o estado de um nó na fila de prioridade.
/// 
/// Atributos:
/// - `node`: Índice do nó no grafo.
/// - `f_score`: Valor da função f(n) = g(n) + h(n
#[derive(PartialEq, PartialOrd)]
struct State {
    node: NodeIndex,
    f_score: f64,
}

impl Eq for State {}
impl Ord for State {
    /// Compara dois estados com base no valor da função f(n).
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_score.partial_cmp(&self.f_score).unwrap()
    }
}