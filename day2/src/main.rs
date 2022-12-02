use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let analyzer = TournamentAnalyzer::new("resources/input_1");
    let total_score = analyzer.score();
    println!("Part 1 solution: {}", total_score);

    let analyzer2 = Part2Analyzer::new("resources/input_1");
    let part_2_score = analyzer2.score();
    println!("Part 2 solution: {}", part_2_score);
}

#[derive(Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn from_char(char: char) -> Self {
        match char {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            _ => panic!("Could not find move for char"),
        }
    }
}

#[derive(Clone)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }

    fn from_char(char: char) -> Self {
        match char {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("Could not find outcome for char"),
        }
    }
}

struct Round {
    opponents_move: Move,
    your_move: Move,
}

impl Round {
    fn outcome(&self) -> Outcome {
        use Move::*;
        use Outcome::*;
        match (&self.opponents_move, &self.your_move) {
            (Paper, Rock) => Lose,
            (Paper, Scissors) => Win,
            (Rock, Paper) => Win,
            (Rock, Scissors) => Lose,
            (Scissors, Paper) => Lose,
            (Scissors, Rock) => Win,
            _ => Draw,
        }
    }

    fn score(&self) -> u32 {
        self.outcome().score() + self.your_move.score()
    }
}

struct TournamentAnalyzer {
    rounds: Vec<Round>,
}

impl TournamentAnalyzer {
    fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let mut rounds = Vec::new();
        for line in reader.lines() {
            let line_content = line.unwrap();
            let strs = line_content.split(" ").collect::<Vec<_>>();
            rounds.push(Round {
                opponents_move: Move::from_char(strs[0].parse::<char>().unwrap()),
                your_move: Move::from_char(strs[1].parse::<char>().unwrap()),
            })
        }

        Self { rounds }
    }

    fn score(&self) -> u32 {
        self.rounds.iter().map(|round| round.score()).sum()
    }
}

#[derive(Clone)]
struct PartialRound {
    opponents_move: Move,
    outcome: Outcome,
}

struct FullRound {
    opponents_move: Move,
    your_move: Move,
    outcome: Outcome,
}

impl FullRound {
    fn from_partial(partial: PartialRound) -> Self {
        use Move::*;
        use Outcome::*;
        let opponents_move = partial.opponents_move;
        let outcome = partial.outcome;
        let your_move = match (opponents_move.clone(), outcome.clone()) {
            (Rock, Win) => Paper,
            (Rock, Lose) => Scissors,
            (Scissors, Win) => Rock,
            (Scissors, Lose) => Paper,
            (Paper, Win) => Scissors,
            (Paper, Lose) => Rock,
            (move_type, Draw) => move_type,
        };
        FullRound {
            your_move,
            opponents_move,
            outcome,
        }
    }
}

impl FullRound {
    fn score(&self) -> u32 {
        self.outcome.score() + self.your_move.score()
    }
}

struct Part2Analyzer {
    partial_rounds: Vec<PartialRound>,
}

impl Part2Analyzer {
    fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let mut partial_rounds = Vec::new();
        for line in reader.lines() {
            let line_content = line.unwrap();
            let strs = line_content.split(" ").collect::<Vec<_>>();
            partial_rounds.push(PartialRound {
                opponents_move: Move::from_char(strs[0].parse::<char>().unwrap()),
                outcome: Outcome::from_char(strs[1].parse::<char>().unwrap()),
            })
        }

        Self { partial_rounds }
    }

    fn score(&self) -> u32 {
        self.partial_rounds
            .iter()
            .map(|partial| FullRound::from_partial(partial.clone()).score())
            .sum()
    }
}
