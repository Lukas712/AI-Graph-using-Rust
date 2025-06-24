mod data_structure;
use data_structure::map::Map;

mod functions;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let origin_city = "São Paulo";
    let destination_city = "Rio de Janeiro";

    let map = Map::new(
        origin_city.to_string(),
        destination_city.to_string(),
    )?;

    map.print_graph_by_levels();

    Ok(())
}
