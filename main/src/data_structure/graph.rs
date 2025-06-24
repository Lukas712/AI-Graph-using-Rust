// src/graph_structure.rs
use super::city::City;
use osmpbfreader::Node;
use petgraph::graph::{Graph, NodeIndex, Neighbors, UnGraph}; // UnGraph para grafos não direcionados
use std::collections::{HashMap};

pub struct GraphStructure {
    graph: Graph<City, f32>,
    city_level_map: HashMap<usize, Vec<NodeIndex>>,

    root: Option<NodeIndex>,
    objective: Option<NodeIndex>,
}

impl GraphStructure {
    pub fn new() -> GraphStructure {
        let mut graph = Graph::new();
        let mut city_level_map = HashMap::new();
        
        GraphStructure {
            graph,
            city_level_map,
            root: None,
            objective: None,
        }
    }

    pub fn insert_city(&mut self, level: usize, city: City) -> NodeIndex{

        let index = self.graph.add_node(city);
        self.city_level_map.entry(level).or_default().push(index);
        return index;
    }

    pub fn add_edge(&mut self, node1: NodeIndex, node2: NodeIndex, weight: f32) {
        self.graph.add_edge(node1, node2, weight);
        
    }

    pub fn get_entire_level(&self, level: usize) -> Option<Vec<NodeIndex>> {
        return self.city_level_map.get(&level).map(|nodes| {
            nodes.clone()
        });
    }

    pub fn get_root(&self) -> Option<NodeIndex> {
        return self.root;
    }

    pub fn get_objective(&self) -> Option<NodeIndex> {
        return self.objective;
    }
    
    pub fn get_neighbors(&self, node: NodeIndex) -> Neighbors<'_, f32> {
        self.graph.neighbors(node)
    }

    pub fn set_root(&mut self, root: NodeIndex) {
        self.root = Some(root);
    }

    pub fn set_objective(&mut self, objective: NodeIndex) {
        self.objective = Some(objective);
    }

    pub fn get_city(&self, index: NodeIndex) -> Option<&City> {
        self.graph.node_weight(index)
    }
}