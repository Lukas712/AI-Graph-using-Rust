use petgraph::graph::NodeIndex;

#[derive(Clone)]
pub struct SearchResult {
    pub path_distance: f64,
    pub visited: i32,
    pub expanded: i32,
    pub avg_branching_factor: f32,
    pub depth: i32,
    pub execution_time: std::time::Duration,
    pub path: Option<Vec<NodeIndex>>,
}