use std::{collections::{BinaryHeap, HashMap}, time::{Instant, Duration}};

use super::common::{get_map_from_file, check_map, FindMode};

pub fn printok() {
    println!("ok");
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct SearchUnit {
    pos: (i32, i32),
    cost: i32,
}

impl Ord for SearchUnit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for SearchUnit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct PathFinder {
    grid_map: Vec<Vec<i32>>,
    start_pt: (i32, i32),
    end_pt: (i32, i32),
} 

impl PathFinder {
    pub fn from_file(map_path: &str, start: (i32, i32), end: (i32, i32)) -> Self {
        Self {
            grid_map: get_map_from_file(map_path),
            start_pt: start,
            end_pt: end,
        }
    }

    fn next_step_nodes(&self, pos: (i32, i32)) -> Vec<SearchUnit> {
        let next_ops = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut next_nodes: Vec<SearchUnit> = Vec::with_capacity(4);

        let map_shape = (self.grid_map[0].len() as i32, self.grid_map.len() as i32);
        next_ops.iter().for_each(|(op_x, op_y)| {
            let next_pos = (pos.0 + op_x, pos.1 + op_y);

            if (0 <= next_pos.0 && next_pos.0 < map_shape.0) 
                && (0 <= next_pos.1 && next_pos.1 < map_shape.1) {
                next_nodes.push(SearchUnit {
                    pos: next_pos,
                    cost: self.grid_map[next_pos.1 as usize][next_pos.0 as usize],
                });
            }
        });
        next_nodes
    }

    pub fn search(&self, mode: FindMode) -> (Vec<(i32, i32)>, i32, Duration) {
        check_map(&self.grid_map);

        let mut queue: BinaryHeap<SearchUnit> = BinaryHeap::new();
        queue.push(SearchUnit { pos: self.start_pt, cost: 0 });

        let mut visited: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut visit_cost: HashMap<(i32, i32), i32> = HashMap::new();
        visited.insert(self.start_pt, (-1, -1));
        visit_cost.insert(self.start_pt, 0);

        let start_time = Instant::now();
        while !queue.is_empty() {
            let searching = queue.pop().unwrap();
            if searching.pos == self.end_pt {
                queue.clear();
                continue;
            }

            let next_nodes = self.next_step_nodes(searching.pos);
            next_nodes.iter().for_each(|next_node| {
                let updated_cost = visit_cost.get(&searching.pos).unwrap() + next_node.cost;

                if !visit_cost.contains_key(&next_node.pos) || &updated_cost < visit_cost.get(&next_node.pos).unwrap() {
                    match mode {
                        FindMode::Dijkstra => queue.push(SearchUnit {
                            ..*next_node
                        }),
                        FindMode::AStar => queue.push(SearchUnit {
                            cost: next_node.cost + (next_node.pos.0 - self.end_pt.0).abs() + 
                                (next_node.pos.1 - self.end_pt.1).abs(),
                            ..*next_node
                        }),
                    }

                    visit_cost.insert(next_node.pos, updated_cost);
                    visited.insert(next_node.pos, searching.pos);
                }
            })
        }
        let end_time = Instant::now();

        let mut found_path: Vec<(i32, i32)> = Vec::new();
        found_path.push(self.end_pt);

        let mut next_node = visited.get(&self.end_pt).unwrap();
        while *next_node != (-1, -1) {
            found_path.push(next_node.clone());
            next_node = visited.get(next_node).unwrap();
        }

        (found_path, *visit_cost.get(&self.end_pt).unwrap(), end_time - start_time)
    }
}
