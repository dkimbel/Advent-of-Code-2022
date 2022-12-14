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
    fn new(s: &str) -> Self {
        let (x, y) = s.split_once(',').unwrap();
        Self {
            x: x.parse::<u32>().unwrap(),
            y: y.parse::<u32>().unwrap(),
        }
    }
}

struct SandSimulator {
    rock_coords: HashSet<Coords>,
}

impl SandSimulator {
    fn simulate(&self) -> u32 {
        todo!()
    }

    fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let mut rock_coords = HashSet::new();

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
                    }
                } else if coords_left.y == coords_right.y {
                    let min_x = cmp::min(coords_left.x, coords_right.x);
                    let max_x = cmp::max(coords_left.x, coords_right.x);
                    let y = coords_left.y;

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

        Self { rock_coords }
    }
}
