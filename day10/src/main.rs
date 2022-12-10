use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let analyzer = SignalAnalyzer::new("resources/input_1");
    let solution_1 = analyzer.signal_score();
    println!("Part 1 solution: {}", solution_1);
}

enum Instruction {
    Noop,
    AddX { x: i32 },
}

impl Instruction {
    fn from_line(line: &str) -> Self {
        if line == "noop" {
            Self::Noop
        } else {
            let (first, second) = line.split_once(' ').unwrap();
            if first == "addx" {
                let x = second.parse::<i32>().unwrap();
                Self::AddX { x }
            } else {
                panic!("Cannot parse instruction from line {}", line)
            }
        }
    }

    fn num_cycles(&self) -> u8 {
        use Instruction::*;
        match self {
            Noop => 1,
            AddX { .. } => 2,
        }
    }
}

struct Tick {
    x_register_during: i32,
}

struct SignalAnalyzer {
    ticks: Vec<Tick>,
}

impl SignalAnalyzer {
    const SIGNAL_SCORE_CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];
    const REGISTER_X_INITIAL: i32 = 1;

    fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let mut ticks = Vec::new();
        let mut register_x = Self::REGISTER_X_INITIAL;

        for line in reader.lines() {
            let instruction = Instruction::from_line(&line.unwrap());
            for _ in 0..instruction.num_cycles() {
                ticks.push(Tick {
                    x_register_during: register_x,
                })
            }
            use Instruction::*;
            match instruction {
                AddX { x } => register_x += x,
                _ => (),
            }
        }

        Self { ticks }
    }

    fn signal_score(&self) -> i32 {
        let mut signal_score = 0;

        for cycle_num in Self::SIGNAL_SCORE_CYCLES {
            let tick_index = cycle_num - 1;
            let tick = &self.ticks[tick_index as usize];
            signal_score += tick.x_register_during * cycle_num;
        }

        signal_score
    }
}
