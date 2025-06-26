use crate::data_structure::city::City;
use crate::data_structure::graph::GraphStructure;

use petgraph::graph::NodeIndex;

pub fn is_between_bounding_box(
    graph: &GraphStructure,
    origin_city: NodeIndex,
    objective_city: NodeIndex,
    city: &City,
) -> bool {
    let origin = graph.get_city(origin_city).unwrap();
    let objective = graph.get_city(objective_city).unwrap();

    let origin_lat = origin.get_latitude();
    let origin_lon = origin.get_longitude();
    let objective_lat = objective.get_latitude();
    let objective_lon = objective.get_longitude();
    let city_lat = city.get_latitude();
    let city_lon = city.get_longitude();

    return (origin_lat - city_lat).abs() <= (objective_lat - origin_lat).abs() &&
    (origin_lon - city_lon).abs() <= (objective_lon - origin_lon).abs()
}