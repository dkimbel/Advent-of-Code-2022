use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() {
    let analyzer = StreamAnalyzer::new("resources/input_1");
    let solution_1 = analyzer.count_til_end_marker().unwrap();
    println!("Part 1 solution: {}", solution_1);

    let solution_2 = analyzer.count_til_start_marker().unwrap();
    println!("Part 2 solution: {}", solution_2);
}

struct StreamAnalyzer {
    stream: String,
}

impl StreamAnalyzer {
    fn new(file_path: &str) -> Self {
        let file_content = fs::read_to_string(file_path).unwrap();
        Self {
            stream: file_content,
        }
    }

    fn count_til_end_marker(&self) -> Option<usize> {
        let chars = self.stream.chars();
        let mut last_four = VecDeque::new();

        for (i, char) in chars.enumerate() {
            last_four.push_front(char);
            if (last_four.len() == 5) {
                last_four.pop_back();
            }

            let mut unique_last_four: HashSet<char> = HashSet::new();
            unique_last_four.extend(last_four.iter());
            if unique_last_four.len() == 4 {
                return Some(i + 1); // convert zero-based index to one-based solution
            }
        }

        None
    }

    fn count_til_start_marker(&self) -> Option<usize> {
        let chars = self.stream.chars();
        let mut last_fourteen = VecDeque::new();

        for (i, char) in chars.enumerate() {
            last_fourteen.push_front(char);
            if (last_fourteen.len() == 15) {
                last_fourteen.pop_back();
            }

            let mut unique_last_fourteen: HashSet<char> = HashSet::new();
            unique_last_fourteen.extend(last_fourteen.iter());
            if unique_last_fourteen.len() == 14 {
                return Some(i + 1); // convert zero-based index to one-based solution
            }
        }

        None
    }
}
