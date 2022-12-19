use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn main() {
    let analyzer = BlueprintAnalyzer::new("resources/input_1");
    println!("{:#?}", analyzer);
}

enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct RobotCost {
    ore: u32,
    clay: u32,
    obsidian: u32,
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_robot_cost: RobotCost,
    clay_robot_cost: RobotCost,
    obsidian_robot_cost: RobotCost,
    geode_robot_cost: RobotCost,
}

#[derive(Debug)]
struct BlueprintAnalyzer {
    blueprints: Vec<Blueprint>,
}

impl BlueprintAnalyzer {
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
