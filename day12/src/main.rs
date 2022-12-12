use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let grid = Grid::new("resources/input_1");
    let solution_1 = grid.fewest_steps_to_goal();
    println!("Part 1 solution: {}", solution_1);
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Coords {
    x: usize,
    y: usize,
}

struct Grid {
    heights: Vec<Vec<u8>>,
    start_coords: Coords,
    goal_coords: Coords,
    max_x: usize,
    max_y: usize,
}

impl Grid {
    fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let mut heights = Vec::new();
        let mut start_coords = None;
        let mut goal_coords = None;

        for (y, line) in reader.lines().enumerate() {
            let mut row = Vec::new();
            for (x, char) in line.unwrap().chars().enumerate() {
                let char = if char == 'S' {
                    start_coords = Some(Coords { x, y });
                    'a'
                } else if char == 'E' {
                    goal_coords = Some(Coords { x, y });
                    'z'
                } else {
                    char
                };
                row.push(Self::char_to_height(char));
            }
            heights.push(row);
        }

        // assumes all rows have equal width
        let max_x = heights[0].len() - 1;
        let max_y = heights.len() - 1;

        Self {
            heights,
            start_coords: start_coords.unwrap(),
            goal_coords: goal_coords.unwrap(),
            max_x,
            max_y,
        }
    }

    // converts 'a' to 1, 'z' to 26
    fn char_to_height(char: char) -> u8 {
        (char.to_digit(36).unwrap() - 9) as u8
    }

    fn height_at_coords(&self, coords: Coords) -> u8 {
        self.heights[coords.y][coords.x]
    }

    // breadth-first search
    fn fewest_steps_to_goal(&self) -> u32 {
        let mut visited: HashSet<Coords> = HashSet::new();
        // tracks num steps taken so far along with coords
        let mut to_visit: VecDeque<(Coords, u32)> = VecDeque::from([(self.start_coords, 0)]);

        while let Some((coords, steps_taken)) = to_visit.pop_front() {
            if visited.contains(&coords) {
                continue;
            }
            visited.insert(coords);
            if coords == self.goal_coords {
                return steps_taken;
            }

            let next_steps_taken = steps_taken + 1;
            let height = self.height_at_coords(coords);
            // maybe add coords above
            if coords.y > 0 {
                let coords_above = Coords {
                    x: coords.x,
                    y: coords.y - 1,
                };
                let height_above = self.height_at_coords(coords_above);
                if height_above <= height + 1 {
                    to_visit.push_back((coords_above, next_steps_taken));
                }
            }
            // maybe add coords below
            if coords.y < self.max_y {
                let coords_below = Coords {
                    x: coords.x,
                    y: coords.y + 1,
                };
                let height_below = self.height_at_coords(coords_below);
                if height_below <= height + 1 {
                    to_visit.push_back((coords_below, next_steps_taken));
                }
            }
            // maybe add coords to left
            if coords.x > 0 {
                let coords_left = Coords {
                    x: coords.x - 1,
                    y: coords.y,
                };
                let height_left = self.height_at_coords(coords_left);
                if height_left <= height + 1 {
                    to_visit.push_back((coords_left, next_steps_taken));
                }
            }
            // maybe add coords to right
            if coords.x < self.max_x {
                let coords_right = Coords {
                    x: coords.x + 1,
                    y: coords.y,
                };
                let height_right = self.height_at_coords(coords_right);
                if height_right <= height + 1 {
                    to_visit.push_back((coords_right, next_steps_taken));
                }
            }
        }
        panic!("Exhausted all possible paths without reaching goal");
    }
}
