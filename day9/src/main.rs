use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut rope_simulator = RopeSimulator::new(2);
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
    knot_locs: Vec<Coords>,
    tail_visited: HashSet<Coords>,
}

impl RopeSimulator {
    fn new(num_knots: usize) -> Self {
        let starting_loc = Coords { x: 0, y: 0 };
        Self {
            knot_locs: vec![starting_loc; num_knots],
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
            let mut new_knot_locs = Vec::new();
            let mut maybe_previous_knot_loc = None;
            for (i, knot_loc) in self.knot_locs.iter().enumerate() {
                let new_knot_loc = if i == 0 {
                    RopeSimulator::new_head_loc(*knot_loc, movement.direction)
                } else {
                    let relative_head_loc = maybe_previous_knot_loc.unwrap();
                    RopeSimulator::new_tail_loc(maybe_previous_knot_loc.unwrap(), *knot_loc)
                };
                maybe_previous_knot_loc = Some(new_knot_loc);
                new_knot_locs.push(new_knot_loc);
            }
            self.knot_locs = new_knot_locs;
            self.tail_visited.insert(maybe_previous_knot_loc.unwrap());
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
