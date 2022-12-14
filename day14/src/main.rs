use std::cmp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let simulator = SandSimulator::new("resources/input_1");
    let sand_units = simulator.simulate();
    println!("Part 1 solution: {}", sand_units);
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Coords {
    x: u32,
    y: u32,
}

impl Coords {
    const INITIAL_SAND: Self = Self { x: 500, y: 0 };

    fn new(s: &str) -> Self {
        let (x, y) = s.split_once(',').unwrap();
        Self {
            x: x.parse::<u32>().unwrap(),
            y: y.parse::<u32>().unwrap(),
        }
    }

    fn after_sand_move(&self) -> (Self, Self, Self) {
        let first_choice = Self {
            x: self.x,
            y: self.y + 1,
        };
        let second_choice = Self {
            x: self.x - 1,
            y: self.y + 1,
        };
        let third_choice = Self {
            x: self.x + 1,
            y: self.y + 1,
        };
        (first_choice, second_choice, third_choice)
    }
}

struct SandSimulator {
    rock_coords: HashSet<Coords>,
    max_y: u32,
}

impl SandSimulator {
    fn simulate(&self) -> usize {
        let mut all_resting_sand_coords = HashSet::new();
        let mut sand_reached_void = false;

        while !sand_reached_void {
            let mut sand_unit_coords = Coords::INITIAL_SAND;
            loop {
                let mut moved = false;
                let (dest0, dest1, dest2) = sand_unit_coords.after_sand_move();
                for dest in [dest0, dest1, dest2] {
                    if !self.rock_coords.contains(&dest) && !all_resting_sand_coords.contains(&dest)
                    {
                        sand_unit_coords = dest;
                        moved = true;
                        break;
                    }
                }
                if !moved {
                    all_resting_sand_coords.insert(sand_unit_coords);
                    break;
                } else if sand_unit_coords.y > self.max_y {
                    sand_reached_void = true;
                    break;
                }
            }
        }

        all_resting_sand_coords.len()
    }

    fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let mut rock_coords = HashSet::new();
        let mut overall_max_y = 0;

        for line in reader.lines() {
            let line_content = line.unwrap();
            let points = line_content
                .split(" -> ")
                .map(Coords::new)
                .collect::<Vec<_>>();
            for coords_pair in points.windows(2) {
                let coords_left = &coords_pair[0];
                let coords_right = &coords_pair[1];

                if coords_left.x == coords_right.x {
                    let min_y = cmp::min(coords_left.y, coords_right.y);
                    let max_y = cmp::max(coords_left.y, coords_right.y);
                    let x = coords_left.x;

                    for y in min_y..=max_y {
                        rock_coords.insert(Coords { x, y });
                        overall_max_y = cmp::max(overall_max_y, y);
                    }
                } else if coords_left.y == coords_right.y {
                    let min_x = cmp::min(coords_left.x, coords_right.x);
                    let max_x = cmp::max(coords_left.x, coords_right.x);
                    let y = coords_left.y;
                    overall_max_y = cmp::max(overall_max_y, y);

                    for x in min_x..=max_x {
                        rock_coords.insert(Coords { x, y });
                    }
                } else {
                    panic!(
                        "Pair of coords matches on neither x nor y: {:?} and {:?}",
                        coords_left, coords_right
                    );
                }
            }
        }

        Self {
            rock_coords,
            max_y: overall_max_y,
        }
    }
}
