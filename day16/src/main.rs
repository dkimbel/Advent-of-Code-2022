use std::cmp;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn main() {
    let searcher = PathSearcher::new("resources/input_1");
    let solution_1 = searcher.find_max_total_flow(30);
    println!("Part 1 solution: {}", solution_1);
}

#[derive(Clone, Debug)]
struct Valve {
    id: String,
    is_open: bool,
    flow_per_minute: u32,
}

impl Valve {
    const ROOT_VALVE_ID: &'static str = "AA";
    const MINUTES_TO_ENTER: u32 = 1;
}

struct SearchState {
    minutes_remaining: u32,
    total_flow: u32,
    flow_per_minute: u32,
    open_valve_ids: HashSet<String>,
    current_valve_id: String,
}

impl SearchState {
    fn new(minutes: u32) -> Self {
        Self {
            minutes_remaining: minutes,
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
    // 'Tunnels' are two-way. First key is lower-alphabetical valve id,
    // second key is higher-alphabetical valve id, integer value is the
    // time in minutes to move through the tunnel (aka the edge's cost)
    tunnel_costs: HashMap<String, HashMap<String, u32>>,
    // We separately track 'tunnel locations' to have constant-time lookup
    // from either of the two linked valves, without recording the tunnel's
    // cost in more than one place.
    tunnel_locations: HashMap<String, HashSet<String>>,
}

impl PathSearcher {
    fn find_max_total_flow(&self, max_minutes: u32) -> u32 {
        let mut best_total_flow = 0;
        let mut search_states = VecDeque::from([SearchState::new(max_minutes)]);

        while let Some(state) = search_states.pop_front() {
            if state.minutes_remaining == 0 {
                best_total_flow = cmp::max(best_total_flow, state.total_flow);
                continue;
            }

            let current_valve = self.valves.get(&state.current_valve_id).unwrap();

            // check if valve can be opened and is possibly worth opening (has nonzero flow rate)
            if current_valve.flow_per_minute != 0
                && !state.open_valve_ids.contains(&state.current_valve_id)
            {
                let mut new_open_valve_ids = state.open_valve_ids.clone();
                new_open_valve_ids.insert(current_valve.id.clone());
                search_states.push_back(SearchState {
                    minutes_remaining: state.minutes_remaining - 1,
                    total_flow: state.total_flow + state.flow_per_minute,
                    flow_per_minute: state.flow_per_minute + current_valve.flow_per_minute,
                    open_valve_ids: new_open_valve_ids,
                    current_valve_id: state.current_valve_id.clone(),
                });
            }

            // add a search state for moving to every possible adjacent room (somewhat naive)
            for adjacent_valve_id in self.tunnel_locations.get(&current_valve.id).unwrap() {
                let minutes_cost =
                    Self::get_cost(&self.tunnel_costs, &current_valve.id, adjacent_valve_id);
                if state.minutes_remaining >= minutes_cost {
                    search_states.push_back(SearchState {
                        minutes_remaining: state.minutes_remaining - minutes_cost,
                        total_flow: state.total_flow + (state.flow_per_minute * minutes_cost),
                        flow_per_minute: state.flow_per_minute,
                        open_valve_ids: state.open_valve_ids.clone(),
                        current_valve_id: state.current_valve_id.clone(),
                    })
                }
            }
        }

        best_total_flow
    }

    // it is probably possible to make this much more performant by doing way less cloning
    fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let re =
            Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnel(s?) lead(s?) to valve(s?) (.+)$")
                .unwrap();
        let mut initial_valves = HashMap::new();
        let mut initial_tunnel_costs = HashMap::new();
        let mut initial_tunnel_locations: HashMap<String, HashSet<String>> = HashMap::new();

        for line in reader.lines() {
            let line_content = line.unwrap();
            let cap = re.captures(&line_content).unwrap();
            let valve_id = String::from(&cap[1]);
            let flow_per_minute = cap[2].parse::<u32>().unwrap();
            let adjacent_valve_ids = cap[6].split(", ").collect::<Vec<_>>();

            initial_valves.insert(
                valve_id.clone(),
                Valve {
                    id: valve_id.clone(),
                    flow_per_minute,
                    is_open: false,
                },
            );

            for adjacent_valve_id in adjacent_valve_ids {
                // update tunnel_costs
                let (first_id, second_id) = Self::sorted_id_pair(&valve_id, adjacent_valve_id);
                // add empty hashmap at first_id if not yet present
                let first_id_entry = initial_tunnel_costs
                    .entry(String::from(first_id))
                    .or_insert(HashMap::new());
                first_id_entry.insert(String::from(second_id), Valve::MINUTES_TO_ENTER);

                // update tunnel_locations
                let first_loc = initial_tunnel_locations
                    .entry(String::from(first_id))
                    .or_insert(HashSet::new());
                first_loc.insert(String::from(second_id));
                let second_loc = initial_tunnel_locations
                    .entry(String::from(second_id))
                    .or_insert(HashSet::new());
                second_loc.insert(String::from(first_id));
            }
        }

        let mut valves = initial_valves.clone();
        let mut tunnel_costs = initial_tunnel_costs.clone();
        let mut tunnel_locations = initial_tunnel_locations.clone();

        // prune valves by eliminating all that have zero flow rates (except root valve);
        // this should make upcoming search much, much faster
        for (_, valve) in initial_valves.iter() {
            if valve.flow_per_minute == 0 && valve.id != Valve::ROOT_VALVE_ID {
                // This is a zero-flow-rate, non-root valve: it will only slow down
                // our search and should be removed. We also need to update our tunnels
                // to directly link the valves that used to link to this one, with
                // appropriately-increased 'costs' (minutes to travel through tunnel).
                let adjacent_valve_ids = &tunnel_locations[&valve.id]
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>();
                let unique_adjacent_pairs = Self::unique_combinations(adjacent_valve_ids);

                // the two ids will already be sorted
                for (adjacent_1_id, adjacent_2_id) in unique_adjacent_pairs {
                    // add cost for the would-be new tunnel directly linking the two adjacent
                    // valves; but if there was already an entry and it's lower, keep that
                    let leg_1_cost = Self::get_cost(&tunnel_costs, adjacent_1_id, &valve.id);
                    let leg_2_cost = Self::get_cost(&tunnel_costs, adjacent_2_id, &valve.id);
                    let summed_cost = leg_1_cost + leg_2_cost;
                    let adj_1_cost_entry = tunnel_costs
                        .entry(String::from(adjacent_1_id))
                        .or_insert(HashMap::new());
                    let adj_2_cost_entry = adj_1_cost_entry
                        .entry(String::from(adjacent_2_id))
                        .or_insert(summed_cost);
                    *adj_2_cost_entry = cmp::min(*adj_2_cost_entry, summed_cost);

                    // add tunnel locations direct between two adjacents
                    tunnel_locations
                        .entry(String::from(adjacent_1_id))
                        .or_insert(HashSet::new())
                        .insert(String::from(adjacent_2_id));
                    tunnel_locations
                        .entry(String::from(adjacent_2_id))
                        .or_insert(HashSet::new())
                        .insert(String::from(adjacent_1_id));

                    // destroy all tunnel_locations referencing valve
                    tunnel_locations
                        .get_mut(adjacent_1_id)
                        .unwrap()
                        .remove(&valve.id);
                    tunnel_locations
                        .get_mut(adjacent_2_id)
                        .unwrap()
                        .remove(&valve.id);

                    // destroy all tunnel_costs referencing valve
                    // todo implement destroy_cost
                    Self::destroy_cost(&mut tunnel_costs, adjacent_1_id, &valve.id);
                    Self::destroy_cost(&mut tunnel_costs, adjacent_2_id, &valve.id);
                    tunnel_costs.remove(&valve.id);
                }
                tunnel_locations.remove(&valve.id);
                tunnel_costs.remove(&valve.id);
                valves.remove(&valve.id);
            }
            // root or non-zero-flowrate valve: requires no modification
        }

        Self {
            valves,
            tunnel_costs,
            tunnel_locations,
        }
    }

    fn destroy_cost(
        tunnel_costs: &mut HashMap<String, HashMap<String, u32>>,
        unsorted_id_1: &str,
        unsorted_id_2: &str,
    ) {
        let (id_1, id_2) = Self::sorted_id_pair(unsorted_id_1, unsorted_id_2);
        tunnel_costs.get_mut(id_1).unwrap().remove(id_2);
    }

    fn get_cost(
        tunnel_costs: &HashMap<String, HashMap<String, u32>>,
        unsorted_id_1: &str,
        unsorted_id_2: &str,
    ) -> u32 {
        let (id_1, id_2) = Self::sorted_id_pair(unsorted_id_1, unsorted_id_2);
        *tunnel_costs.get(id_1).unwrap().get(id_2).unwrap()
    }

    fn sorted_id_pair<'a>(id1: &'a str, id2: &'a str) -> (&'a str, &'a str) {
        if id1.cmp(id2) == cmp::Ordering::Greater {
            (id2, id1)
        } else {
            (id1, id2)
        }
    }

    // naive
    fn unique_combinations(valve_ids: &[String]) -> HashSet<(&str, &str)> {
        let mut results = HashSet::new();
        for v1 in valve_ids {
            for v2 in valve_ids {
                if v1 != v2 {
                    results.insert(Self::sorted_id_pair(v1, v2));
                }
            }
        }
        results
    }
}
