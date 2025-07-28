// src/graph_structure.rs
use super::city::City;
use petgraph::graph::{Graph, NodeIndex, Neighbors};
use std::collections::{HashMap};

/// Estrutura que representa o grafo do mapa
/// 
/// Atributos:
/// - `graph`: Um grafo que contém as cidades e os caminhos entre elas.
/// - `city_level_map`: Um mapa que associa cada nível a um vetor de índices de
/// - `root`: O índice do nó raiz do grafo, representando a cidade de origem.
/// - `objective`: O índice do nó objetivo do grafo, representando a cidade de
pub struct GraphStructure {
    graph: Graph<City, f64>,
    city_level_map: HashMap<usize, Vec<NodeIndex>>,

    root: Option<NodeIndex>,
    objective: Option<NodeIndex>,
}

impl GraphStructure {
    /// Construtor para criar um novo grafo vazio.
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

    /// Insere uma cidade no grafo e associa-a a um nível.
    pub fn insert_city(&mut self, level: usize, city: City) -> NodeIndex{

        let index = self.graph.add_node(city);
        self.city_level_map.entry(level).or_default().push(index);
        return index;
    }


    /// Adiciona uma aresta entre dois nós no grafo com um peso especificado.
    pub fn add_edge(&mut self, node1: NodeIndex, node2: NodeIndex, weight: f64) {
        self.graph.add_edge(node1, node2, weight);
        
    }

    /// Retorna todas as cidades de um determinado nível.
    pub fn get_entire_level(&self, level: usize) -> Option<Vec<NodeIndex>> {
        return self.city_level_map.get(&level).map(|nodes| {
            nodes.clone()
        });
    }

    /// Retorna o peso da aresta entre dois nós, se existir.
    pub fn get_edge_weight(&self, node1: NodeIndex, node2: NodeIndex) -> f64 {
        self.graph.edge_weight(self.graph.find_edge(node1, node2).unwrap())
            .cloned()
            .unwrap_or(f64::INFINITY)
    }


    /// Retorna a raiz do grafo, que é a cidade de origem.
    pub fn get_root(&self) -> Option<NodeIndex> {
        return self.root;
    }

    /// Retorna o objetivo do grafo, que é a cidade de destino.
    pub fn get_objective(&self) -> Option<NodeIndex> {
        return self.objective;
    }

    /// Retorna os vizinhos de um nó específico.
    pub fn get_neighbors(&self, node: NodeIndex) -> Neighbors<'_, f64> {
        self.graph.neighbors(node)
    }

    /// Define a raiz do grafo.
    pub fn set_root(&mut self, root: NodeIndex) {
        self.root = Some(root);
    }

    /// Define o objetivo do grafo.
    pub fn set_objective(&mut self, objective: NodeIndex) {
        self.objective = Some(objective);
    }

    /// Retorna uma referência à cidade associada a um índice de nó.
    pub fn get_city(&self, index: NodeIndex) -> Option<&City> {
        self.graph.node_weight(index)
    }
}