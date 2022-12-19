use std::cmp;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn main() {
    let analyzer = BlueprintAnalyzer::new("resources/input_1");
    let solution_1 = analyzer.total_quality_level(24);
    println!("Part 1 solution: {}", solution_1);
}

enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct RobotCost {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

impl RobotCost {
    fn can_afford(&self, ore: usize, clay: usize, obsidian: usize) -> bool {
        ore >= self.ore && clay >= self.clay && obsidian >= self.obsidian
    }
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_robot_cost: RobotCost,
    clay_robot_cost: RobotCost,
    obsidian_robot_cost: RobotCost,
    geode_robot_cost: RobotCost,
}

struct SearchState {
    minutes_remaining: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
}

impl SearchState {
    fn new(minutes: usize) -> Self {
        Self {
            minutes_remaining: minutes,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }
}

impl Blueprint {
    fn quality_level(&self, minutes: usize) -> usize {
        self.max_geodes(minutes) * self.id
    }

    fn max_geodes(&self, minutes: usize) -> usize {
        let mut max_geodes = 0;
        let mut search_states = VecDeque::from([SearchState::new(minutes)]);

        while let Some(state) = search_states.pop_front() {
            if state.minutes_remaining == 0 {
                max_geodes = cmp::max(max_geodes, state.geodes);
                continue;
            }

            let ore_next = state.ore + state.ore_robots;
            let clay_next = state.clay + state.clay_robots;
            let obsidian_next = state.obsidian + state.obsidian_robots;
            let geodes_next = state.geodes + state.geode_robots;
            let minutes_next = state.minutes_remaining - 1;

            // for now, only heuristics are:
            //   - absolutely build a geode robot, without even trying any alternate actions, if
            //     we can afford one
            //   - don't build any resource-specific robot if you have enough of that resource
            //     already to build a geode robot
            // todo reduce code repetition
            if self
                .geode_robot_cost
                .can_afford(state.ore, state.clay, state.obsidian)
            {
                search_states.push_back(SearchState {
                    minutes_remaining: minutes_next,
                    ore: ore_next - self.geode_robot_cost.ore,
                    clay: clay_next - self.geode_robot_cost.clay,
                    obsidian: obsidian_next - self.geode_robot_cost.obsidian,
                    geodes: geodes_next,
                    ore_robots: state.ore_robots,
                    clay_robots: state.clay_robots,
                    obsidian_robots: state.obsidian_robots,
                    geode_robots: state.geode_robots + 1,
                });
                continue;
            }
            if self
                .ore_robot_cost
                .can_afford(state.ore, state.clay, state.obsidian)
                && state.ore < self.geode_robot_cost.ore
            {
                search_states.push_back(SearchState {
                    minutes_remaining: minutes_next,
                    ore: ore_next - self.ore_robot_cost.ore,
                    clay: clay_next - self.ore_robot_cost.clay,
                    obsidian: obsidian_next - self.ore_robot_cost.obsidian,
                    geodes: geodes_next,
                    ore_robots: state.ore_robots + 1,
                    clay_robots: state.clay_robots,
                    obsidian_robots: state.obsidian_robots,
                    geode_robots: state.geode_robots,
                });
            }
            if self
                .clay_robot_cost
                .can_afford(state.ore, state.clay, state.obsidian)
                && state.clay < self.geode_robot_cost.clay
            {
                search_states.push_back(SearchState {
                    minutes_remaining: minutes_next,
                    ore: ore_next - self.clay_robot_cost.ore,
                    clay: clay_next - self.clay_robot_cost.clay,
                    obsidian: obsidian_next - self.clay_robot_cost.obsidian,
                    geodes: geodes_next,
                    ore_robots: state.ore_robots,
                    clay_robots: state.clay_robots + 1,
                    obsidian_robots: state.obsidian_robots,
                    geode_robots: state.geode_robots,
                });
            }
            if self
                .obsidian_robot_cost
                .can_afford(state.ore, state.clay, state.obsidian)
                && state.obsidian < self.geode_robot_cost.obsidian
            {
                search_states.push_back(SearchState {
                    minutes_remaining: minutes_next,
                    ore: ore_next - self.obsidian_robot_cost.ore,
                    clay: clay_next - self.obsidian_robot_cost.clay,
                    obsidian: obsidian_next - self.obsidian_robot_cost.obsidian,
                    geodes: geodes_next,
                    ore_robots: state.ore_robots,
                    clay_robots: state.clay_robots,
                    obsidian_robots: state.obsidian_robots + 1,
                    geode_robots: state.geode_robots,
                });
            }
            // no-op case
            search_states.push_back(SearchState {
                minutes_remaining: minutes_next,
                ore: ore_next,
                clay: clay_next,
                obsidian: obsidian_next,
                geodes: geodes_next,
                ore_robots: state.ore_robots,
                clay_robots: state.clay_robots,
                obsidian_robots: state.obsidian_robots,
                geode_robots: state.geode_robots,
            });
        }

        max_geodes
    }
}

#[derive(Debug)]
struct BlueprintAnalyzer {
    blueprints: Vec<Blueprint>,
}

impl BlueprintAnalyzer {
    fn total_quality_level(&self, minutes: usize) -> usize {
        self.blueprints
            .iter()
            .map(|bp| bp.quality_level(minutes))
            .sum()
    }

    fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let re =
            Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$")
                .unwrap();

        let mut blueprints = Vec::new();

        for line in reader.lines() {
            let line_content = line.unwrap();
            let cap = re.captures(&line_content).unwrap();
            blueprints.push(Blueprint {
                id: cap[1].parse().unwrap(),
                ore_robot_cost: RobotCost {
                    ore: cap[2].parse().unwrap(),
                    clay: 0,
                    obsidian: 0,
                },
                clay_robot_cost: RobotCost {
                    ore: cap[3].parse().unwrap(),
                    clay: 0,
                    obsidian: 0,
                },
                obsidian_robot_cost: RobotCost {
                    ore: cap[4].parse().unwrap(),
                    clay: cap[5].parse().unwrap(),
                    obsidian: 0,
                },
                geode_robot_cost: RobotCost {
                    ore: cap[6].parse().unwrap(),
                    clay: 0,
                    obsidian: cap[7].parse().unwrap(),
                },
            });
        }

        Self { blueprints }
    }
}
