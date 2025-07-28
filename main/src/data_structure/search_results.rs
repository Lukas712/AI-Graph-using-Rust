use petgraph::graph::NodeIndex;


/// Estrutura para armazenar o resultado da busca.
/// 
/// Atributos:
/// - `path_distance`: Distância total do caminho encontrado.
/// - `visited`: Número de nós visitados.
/// - `expanded`: Número de nós expandidos.
/// - `avg_branching_factor`: Fator de ramificação médio.
/// - `depth`: Profundidade máxima alcançada.
/// - `execution_time`: Tempo total de execução da busca.
/// - `path`: Caminho encontrado, se houver.
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