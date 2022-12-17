use std::collections::{HashMap, HashSet};

fn main() {
    let searcher = PathSearcher::new("resources/input_1");
    let solution_1 = searcher.find_max_total_flow();
    println!("Part 1 solution: {}", solution_1);
}

struct Room<'a> {
    id: &'a str,
    is_open: bool,
    flow_per_minute: u32,
    adjacent_rooms: Vec<AdjacentRoom<'a>>,
}

struct AdjacentRoom<'a> {
    id: &'a str,
    minutes_to_enter: u32,
}

struct SearchState<'a> {
    minutes_remaining: u32,
    total_flow: u32,
    flow_per_minute: u32,
    open_valve_room_ids: HashSet<&'a str>,
    current_room_id: &'a str,
}

impl<'a> SearchState<'a> {
    fn new(root_room_id: &'a str) -> Self {
        Self {
            minutes_remaining: 0,
            total_flow: 0,
            flow_per_minute: 0,
            open_valve_room_ids: HashSet::new(),
            current_room_id: root_room_id,
        }
    }
}

enum Action<'a> {
    EnterRoom(&'a str),
    OpenValve,
}

struct PathSearcher<'a> {
    rooms: HashMap<&'a str, Room<'a>>,
}

impl PathSearcher<'_> {
    fn new(file_path: &str) -> Self {
        // parse input file into `rooms`
        // prune rooms by eliminating all zero-flow-rates (except root room)
        todo!()
    }

    fn find_max_total_flow(&self) -> u32 {
        todo!()
    }
}
