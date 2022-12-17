use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn main() {
    let searcher = PathSearcher::new("resources/input_1");
    let solution_1 = searcher.find_max_total_flow();
    println!("Part 1 solution: {}", solution_1);
}

struct Valve {
    id: String,
    is_open: bool,
    flow_per_minute: u32,
    adjacent_valves: Vec<AdjacentValve>,
}

impl Valve {
    const ROOT_VALVE_ID: &'static str = "AA";
}

struct AdjacentValve {
    id: String,
    minutes_to_enter: u32,
}

struct SearchState {
    minutes_remaining: u32,
    total_flow: u32,
    flow_per_minute: u32,
    open_valve_ids: HashSet<String>,
    current_valve_id: String,
}

impl SearchState {
    fn new() -> Self {
        Self {
            minutes_remaining: 0,
            total_flow: 0,
            flow_per_minute: 0,
            open_valve_ids: HashSet::new(),
            current_valve_id: String::from(Valve::ROOT_VALVE_ID),
        }
    }
}

enum Action<'a> {
    EnterValveRoom(&'a str),
    OpenValve,
}

struct PathSearcher {
    valves: HashMap<String, Valve>,
}

impl PathSearcher {
    fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let re =
            Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnel(s?) lead(s?) to valve(s?) (.+)$")
                .unwrap();
        let mut valves = HashMap::new();

        for line in reader.lines() {
            let line_content = line.unwrap();
            println!("{}", line_content);
            // todo fix panic
            let cap = re.captures(&line_content).unwrap();
            let valve_id = String::from(&cap[1]);
            let flow_per_minute = cap[2].parse::<u32>().unwrap();
            let adjacent_valves = cap[6]
                .split(", ")
                .map(|valve_id| AdjacentValve {
                    id: String::from(valve_id),
                    minutes_to_enter: 0,
                })
                .collect::<Vec<_>>();

            valves.insert(
                valve_id.clone(),
                Valve {
                    id: valve_id,
                    flow_per_minute,
                    adjacent_valves,
                    is_open: false,
                },
            );
        }

        // prune valves by eliminating all that have zero flow rates (except root valve);
        // this should make upcoming search much, much faster
        // todo implement
        Self { valves }
    }

    fn find_max_total_flow(&self) -> u32 {
        todo!()
    }
}
