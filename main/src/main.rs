mod data_structure;
use data_structure::map::Map;

mod functions;

mod informed_algorithms;
use informed_algorithms::backtracking::backtracking;
use informed_algorithms::breadth_first_search::breadth_first_search;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let origin_city = "Eugenópolis";
    let destination_city = "Muriaé";

    let map = Map::new(
        origin_city.to_string(),
        destination_city.to_string(),
    )?;

    map.print_graph_by_levels();
    // backtracking(map.get_graph().clone())
    //     .map(|path| {
    //         println!("Caminho encontrado: {:?}", path);
    //     })
    //     .unwrap_or_else(|| {
    //         println!("Nenhum caminho encontrado.");
    //     });
    breadth_first_search(map.get_graph().clone())
        .map(|path| {
            println!("Caminho encontrado: {:?}", path);
        })
        .unwrap_or_else(|| {
            println!("Nenhum caminho encontrado.");
        });

    Ok(())
}
