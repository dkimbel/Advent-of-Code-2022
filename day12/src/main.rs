use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let grid = Grid::new("resources/input_1");
}

struct Coords {
    x: u8,
    y: u8,
}

struct Grid {
    heights: Vec<Vec<u8>>,
    start_coords: Coords,
    goal_coords: Coords,
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
                    start_coords = Some(Coords {
                        x: x as u8,
                        y: y as u8,
                    });
                    'a'
                } else if char == 'E' {
                    goal_coords = Some(Coords {
                        x: x as u8,
                        y: y as u8,
                    });
                    'z'
                } else {
                    char
                };
                row.push(Self::char_to_height(char));
            }
            heights.push(row);
        }

        Self {
            heights,
            start_coords: start_coords.unwrap(),
            goal_coords: goal_coords.unwrap(),
        }
    }

    // converts 'a' to 1, 'z' to 26
    fn char_to_height(char: char) -> u8 {
        (char.to_digit(36).unwrap() - 9) as u8
    }
}
