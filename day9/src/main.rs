use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut rope_simulator = RopeSimulator::new();
    rope_simulator.simulate_from_file("resources/input_1");
    let solution_1 = rope_simulator.num_spaces_tail_visited();
    println!("Part 1 solution: {}", solution_1);
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
struct Coords {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(char: char) -> Self {
        use Direction::*;
        match char {
            'U' => Up,
            'D' => Down,
            'L' => Left,
            'R' => Right,
            _ => panic!("Cannot parse char {} as a direction", char),
        }
    }
}

struct Movement {
    direction: Direction,
    num_spaces: u8,
}

impl Movement {
    fn from_line(file_line: &str) -> Self {
        let split = file_line.split(' ').collect::<Vec<_>>();
        let direction = Direction::from_char(split[0].parse::<char>().unwrap());
        let num_spaces = split[1].parse::<u8>().unwrap();

        Self {
            direction,
            num_spaces,
        }
    }
}

struct RopeSimulator {
    head_loc: Coords,
    tail_loc: Coords,
    tail_visited: HashSet<Coords>,
}

impl RopeSimulator {
    fn new() -> Self {
        let starting_loc = Coords { x: 0, y: 0 };
        Self {
            head_loc: starting_loc,
            tail_loc: starting_loc,
            tail_visited: HashSet::from([starting_loc]),
        }
    }

    fn num_spaces_tail_visited(&self) -> usize {
        self.tail_visited.len()
    }

    fn simulate_from_file(&mut self, file_path: &str) {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let movement = Movement::from_line(&line.unwrap());
            self.simulate_movement(movement);
        }
    }

    fn simulate_movement(&mut self, movement: Movement) {
        let mut spaces_to_move = movement.num_spaces;

        while spaces_to_move > 0 {
            spaces_to_move -= 1;
            self.head_loc = RopeSimulator::new_head_loc(self.head_loc, movement.direction);
            self.tail_loc = RopeSimulator::new_tail_loc(self.head_loc, self.tail_loc);
            self.tail_visited.insert(self.tail_loc);
        }
    }

    fn new_head_loc(head_loc: Coords, direction: Direction) -> Coords {
        use Direction::*;
        match direction {
            Up => Coords {
                x: head_loc.x,
                y: head_loc.y + 1,
            },
            Down => Coords {
                x: head_loc.x,
                y: head_loc.y - 1,
            },
            Left => Coords {
                x: head_loc.x - 1,
                y: head_loc.y,
            },
            Right => Coords {
                x: head_loc.x + 1,
                y: head_loc.y,
            },
        }
    }

    fn new_tail_loc(head_loc: Coords, tail_loc: Coords) -> Coords {
        let x_diff = head_loc.x - tail_loc.x;
        let y_diff = head_loc.y - tail_loc.y;

        if x_diff.abs() > 2 || y_diff.abs() > 2 || (x_diff.abs() + y_diff.abs() > 3) {
            panic!(
                "Illegal rope position! Head {:?} and tail {:?} are too far apart",
                head_loc, tail_loc
            )
        }
        if x_diff == -2 {
            Coords {
                x: tail_loc.x - 1,
                y: head_loc.y, // automatically deal with diagonal
            }
        } else if x_diff == 2 {
            Coords {
                x: tail_loc.x + 1,
                y: head_loc.y, // automatically deal with diagonal
            }
        } else if y_diff == -2 {
            Coords {
                x: head_loc.x, // automatically deal with diagonal
                y: tail_loc.y - 1,
            }
        } else if y_diff == 2 {
            Coords {
                x: head_loc.x, // automatically deal with diagonal
                y: tail_loc.y + 1,
            }
        } else {
            tail_loc
        }
    }
}
