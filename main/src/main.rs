mod data_structure;
use data_structure::map::Map;

mod functions;

mod uninformed_algorithms;
use uninformed_algorithms::backtracking::backtracking;
use uninformed_algorithms::breadth_first_search::breadth_first_search;
use uninformed_algorithms::ordered_search::ordered_search;
use uninformed_algorithms::deep_search::depth_limited_search;

mod informed_algorithms;
use informed_algorithms::greedy_search::greedy_search;
use informed_algorithms::a_star::a_star_search;
use informed_algorithms::ida_star::ida_star_search;

use crate::data_structure::city;
use crate::data_structure::search_results::SearchResult;
use crate::functions::path_distance::calculate_path_distance;
use crate::functions::results::{print_no_result, print_path};


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let origin_city = "Muriaé";
    let destination_city = "Eugenópolis";

    let map = Map::new(
        origin_city.to_string(),
        destination_city.to_string(),
    )?;

    map.print_graph_by_levels();

    // backtracking(map.get_graph().clone())
    // .map(|search_result| {
    //     print_path(search_result, &map);
    // })
    // .unwrap_or_else(|| {
    //     print_no_result();
    // });
    
    // breadth_first_search(map.get_graph().clone())
    // .map(|search_result| {
    //     print_path(search_result, &map);
    // })
    // .unwrap_or_else(|| {
    //     print_no_result();
    // });
    
    // ordered_search(map.get_graph().clone())
    // .map(|search_result| {
    //     print_path(search_result, &map);
    // })
    // .unwrap_or_else(|| {
    //     print_no_result();
    // });

    // greedy_search(map.get_graph().clone())
    // .map(|search_result| {
    //     print_path(search_result, &map);
    // })
    // .unwrap_or_else(|| {
    //     print_no_result();
    // });

    // a_star_search(map.get_graph().clone())
    // .map(|search_result| {
    //     print_path(search_result, &map);
    // })
    // .unwrap_or_else(|| {
    //     print_no_result();
    // });

    ida_star_search(map.get_graph().clone())
    .map(|search_result| {
        print_path(search_result, &map);
    })
    .unwrap_or_else(|| {
        print_no_result();
    });

    // (map.get_graph().clone())
    // .map(|search_result| {
    //     print_path(search_result, &map);
    // })
    // .unwrap_or_else(|| {
    //     print_no_result();
    // });

    Ok(())
}
