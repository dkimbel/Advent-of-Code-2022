use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let analyzer = RangeAnalyzer::new("resources/input_1");
    let solution_1 = analyzer.count_full_overlaps();
    println!("Part 1 solution: {}", solution_1);
    let solution_2 = analyzer.count_partial_overlaps();
    println!("Part 2 solution: {}", solution_2);
}

struct Pair {
    start: u32,
    end: u32,
}

impl Pair {
    // example input: "2-4"
    fn new(input: &str) -> Self {
        let mut split = input.split("-");
        let (start, end) = (split.next().unwrap(), split.next().unwrap());
        Self {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }
}

struct RangePair {
    first: Pair,
    second: Pair,
}

impl RangePair {
    fn full_overlap(&self) -> bool {
        let first_contains_second =
            self.first.start <= self.second.start && self.first.end >= self.second.end;
        let second_contains_first =
            self.second.start <= self.first.start && self.second.end >= self.first.end;
        first_contains_second || second_contains_first
    }

    fn any_overlap(&self) -> bool {
        !(self.first.start > self.second.end || self.second.start > self.first.end)
    }

    // example input: "2-4,3-5"
    fn new(file_line: &str) -> Self {
        let mut split = file_line.split(",");
        let (first, second) = (split.next().unwrap(), split.next().unwrap());
        Self {
            first: Pair::new(first),
            second: Pair::new(second),
        }
    }
}

struct RangeAnalyzer {
    range_pairs: Vec<RangePair>,
}

impl RangeAnalyzer {
    fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let mut range_pairs = Vec::new();
        for line in reader.lines() {
            range_pairs.push(RangePair::new(&line.unwrap()));
        }

        Self { range_pairs }
    }

    fn count_full_overlaps(&self) -> u32 {
        self.range_pairs
            .iter()
            .map(|pair| if pair.full_overlap() { 1 } else { 0 })
            .sum()
    }

    fn count_partial_overlaps(&self) -> u32 {
        self.range_pairs
            .iter()
            .map(|pair| if pair.any_overlap() { 1 } else { 0 })
            .sum()
    }
}

// find a more idomatic way to count than having map convert to int
