use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn main() {
    let analyzer = SensorAnalyzer::new("resources/input_1");
    // let solution_1 = analyzer.num_coords_in_range_on_row(2_000_000);
    let solution_1 = analyzer.num_coords_in_range_on_row(10);
    println!("Part 1 solution: {}", solution_1);
    // let distress_beacon = analyzer.find_beacon_in_range(0, 4_000_000);
    let distress_beacon = analyzer.find_beacon_in_range(0, 20);
    let solution_2 = distress_beacon.tuning_frequency();
    println!("Part 2 solution: {}", solution_2);
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Coords {
    x: i32,
    y: i32,
}

struct XRange {
    min: i32,
    max: i32,
}

impl XRange {
    fn trim_between(&self, min_allowed: i32, max_allowed: i32) -> Option<Self> {
        let new_min = cmp::max(self.min, min_allowed);
        let new_max = cmp::min(self.max, max_allowed);
        if new_min > new_max {
            None
        } else {
            Some(Self {
                min: new_min,
                max: new_max,
            })
        }
    }
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

    fn range_in_row(&self, y: i32) -> Option<XRange> {
        let y_distance = (self.coords.y - y).abs();
        let available_x_distance = self.nearest_beacon_manhattan - y_distance;
        if available_x_distance <= 0 {
            None
        } else {
            Some(XRange {
                min: self.coords.x - (available_x_distance / 2),
                max: self.coords.x + (available_x_distance / 2),
            })
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Beacon {
    coords: Coords,
}

impl Beacon {
    fn tuning_frequency(&self) -> u64 {
        self.coords.x as u64 * 4_000_000 + self.coords.y as u64
    }
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
    fn find_beacon_in_range(&self, min_coord: i32, max_coord: i32) -> Beacon {
        for y in min_coord..=max_coord {
            let mut x_ranges = self
                .sensors
                .iter()
                .filter_map(|sensor| sensor.range_in_row(y))
                .filter_map(|xrange| xrange.trim_between(min_coord, max_coord))
                .collect::<Vec<_>>();

            // sort ranges by their minimum value ascending
            x_ranges.sort_by(|r0, r1| r0.min.cmp(&r1.min));

            let mut maybe_max_x = None;
            for x_range in x_ranges {
                if let Some(max_x) = maybe_max_x {
                    if x_range.min > max_x {
                        if x_range.min - max_x > 1 {
                            panic!("Found more than one empty spot for beacon!")
                        } else {
                            return Beacon {
                                coords: Coords { x: max_x + 1, y },
                            };
                        }
                    }
                    maybe_max_x = Some(cmp::max(max_x, x_range.max));
                } else {
                    maybe_max_x = Some(x_range.max);
                }
            }
        }
        panic!("Could not find any empty spot for beacon!")
    }

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
