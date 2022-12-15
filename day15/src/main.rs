use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn main() {
    let analyzer = SensorAnalyzer::new("resources/input_1");
    let solution_1 = analyzer.num_coords_in_range_on_row(2_000_000);
    println!("Part 1 solution: {}", solution_1);
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Coords {
    x: i32,
    y: i32,
}

struct Sensor {
    coords: Coords,
    nearest_beacon_manhattan: i32,
}

impl Sensor {
    fn in_range(&self, coords: &Coords) -> bool {
        let manhattan = (coords.x - self.coords.x).abs() + (coords.y - self.coords.y).abs();
        // if manhattan were zero, sensor would be occupying space; doesn't count as in range
        manhattan <= self.nearest_beacon_manhattan && manhattan != 0
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Beacon {
    coords: Coords,
}

struct SensorAnalyzer {
    beacons: HashSet<Beacon>,
    sensors: Vec<Sensor>,
    min_x_in_range: i32,
    max_x_in_range: i32,
    min_y_in_range: i32,
    max_y_in_range: i32,
}

impl SensorAnalyzer {
    fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let re = Regex::new(
            r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$",
        )
        .unwrap();

        let mut beacons = HashSet::new();
        let mut sensors = Vec::new();
        let mut min_x_in_range = None;
        let mut max_x_in_range = None;
        let mut min_y_in_range = None;
        let mut max_y_in_range = None;

        for line in reader.lines() {
            let line_content = line.unwrap();
            let cap = re.captures(&line_content).unwrap();
            let sensor_x = cap[1].parse::<i32>().unwrap();
            let sensor_y = cap[2].parse::<i32>().unwrap();
            let beacon_x = cap[3].parse::<i32>().unwrap();
            let beacon_y = cap[4].parse::<i32>().unwrap();

            beacons.insert(Beacon {
                coords: Coords {
                    x: beacon_x,
                    y: beacon_y,
                },
            });

            let manhattan = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
            sensors.push(Sensor {
                coords: Coords {
                    x: sensor_x,
                    y: sensor_y,
                },
                nearest_beacon_manhattan: manhattan,
            });

            let min_x = sensor_x - manhattan;
            min_x_in_range = match min_x_in_range {
                None => Some(min_x),
                Some(x) => Some(cmp::min(x, min_x)),
            };
            let max_x = sensor_x + manhattan;
            max_x_in_range = match max_x_in_range {
                None => Some(max_x),
                Some(x) => Some(cmp::max(x, max_x)),
            };
            let min_y = sensor_y - manhattan;
            min_y_in_range = match min_y_in_range {
                None => Some(min_y),
                Some(y) => Some(cmp::min(y, min_y)),
            };
            let max_y = sensor_y + manhattan;
            max_y_in_range = match max_y_in_range {
                None => Some(max_y),
                Some(y) => Some(cmp::max(y, max_y)),
            };
        }

        Self {
            beacons,
            sensors,
            min_x_in_range: min_x_in_range.unwrap(),
            max_x_in_range: max_x_in_range.unwrap(),
            min_y_in_range: min_y_in_range.unwrap(),
            max_y_in_range: max_y_in_range.unwrap(),
        }
    }

    fn num_coords_in_range_on_row(&self, y: i32) -> u32 {
        let mut num_coords_in_range = 0;

        for x in self.min_x_in_range..=self.max_x_in_range {
            let coords = Coords { x, y };
            let possible_beacon = Beacon { coords };
            if self.beacons.contains(&possible_beacon) {
                continue; // this spot is already occupied by a beacon
            }
            for sensor in &self.sensors {
                if sensor.in_range(&coords) {
                    num_coords_in_range += 1;
                    break;
                }
            }
        }

        num_coords_in_range
    }
}
