use crate::data_structure::graph::GraphStructure;
use petgraph::graph::NodeIndex;

pub fn calculate_path_distance(graph: &GraphStructure, path: Vec<NodeIndex>) -> f64 {
    let mut total_distance = 0.0;
    
    for i in 0..path.len() - 1 {
        let current_node = path[i];
        let next_node = path[i + 1];
        
        let weight = graph.get_edge_weight(current_node, next_node);
        println!(
            "  {:?} -> {:?} = {}",
            graph.get_city(current_node).unwrap().get_name(),
            graph.get_city(next_node).unwrap().get_name(),
            weight
        );
        
        total_distance += weight;
    }
    
    total_distance
}