use std::{fs::File, io::Read};

pub enum FindMode {
    Dijkstra,
    AStar,
}

pub fn check_map(map_vec: &Vec<Vec<i32>>) {
    let vec_len = map_vec[0].len();
    map_vec.iter().for_each(|v| {
        assert_eq!(vec_len, v.len());
    })
}

pub fn get_map_from_file(filename: &str) -> Vec<Vec<i32>> {
    let mut file = File::open(filename).unwrap();
    let mut map_buf = String::new();
    file.read_to_string(&mut map_buf).unwrap();

    map_buf.split("\n").map(|v| 
        v.split(" ").map(|n| 
            n.parse::<i32>().unwrap()
        ).collect::<Vec<_>>()
    ).collect::<Vec<_>>()
}