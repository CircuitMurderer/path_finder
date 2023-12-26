use finder::path_finder::{printok, PathFinder};

use crate::finder::common::FindMode;

mod finder;

fn main() {
    printok();

    let finder = PathFinder::from_file("data/map.txt", (0, 7), (22, 7));
    let res = finder.search(FindMode::Dijkstra);

    println!("=== Dijkstra ===");
    println!("The found path: {:?}", res.0);
    println!("The total cost: {}", res.1);
    println!("Duration: {:?}", res.2);

    let finder = PathFinder::from_file("data/map.txt", (0, 7), (22, 7));
    let res = finder.search(FindMode::AStar);

    println!();
    println!("=== A Star ===");
    println!("The found path: {:?}", res.0);
    println!("The total cost: {}", res.1);
    println!("Duration: {:?}", res.2);
}
