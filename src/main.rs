use finder::path_finder::PathFinder;

use crate::finder::common::FindMode;

mod finder;

fn main() {
    let finder = PathFinder::from_file("data/map.txt", (0, 7), (22, 7));
    let res = finder.search(FindMode::Dijkstra);

    println!("=== Dijkstra ===");
    println!("The found path: {:?}", res.0);
    println!("The total cost: {}", res.1);
    println!("Time cost: {:?}", res.2);
    println!("Space cost: {} + {} + {} units\n", res.3.0, res.3.1, res.3.2);

    let finder = PathFinder::from_file("data/map.txt", (0, 7), (22, 7));
    let res = finder.search(FindMode::AStar);

    println!("===  A Star  ===");
    println!("The found path: {:?}", res.0);
    println!("The total cost: {}", res.1);
    println!("Time cost: {:?}", res.2);
    println!("Space cost: {} + {} + {} units\n", res.3.0, res.3.1, res.3.2);
}
