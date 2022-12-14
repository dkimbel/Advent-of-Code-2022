use regex::Regex;
use std::collections::VecDeque;
use std::fs;

fn main() {
    let mut tracker = StackTracker::new("resources/input_1");
    tracker.execute_all();
    let solution_1 = tracker.top_chars();
    println!("Part 1 solution: {}", solution_1);

    let mut tracker2 = StackTracker::new("resources/input_1");
    tracker2.execute_all_multi_crate_move();
    let solution_2 = tracker2.top_chars();
    println!("Part 2 solution: {}", solution_2);
}

#[derive(Clone)]
struct Command {
    num_crates: usize,
    // indexes are zero-based
    source_stack_index: usize,
    dest_stack_index: usize,
}

impl Command {
    fn new(file_line: &str) -> Self {
        let pattern: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        let cap = pattern.captures(file_line).unwrap();
        let num_crates = cap[1].parse::<usize>().unwrap();
        let source_stack_num = cap[2].parse::<usize>().unwrap();
        let dest_stack_num = cap[3].parse::<usize>().unwrap();

        Self {
            num_crates,
            source_stack_index: source_stack_num - 1,
            dest_stack_index: dest_stack_num - 1,
        }
    }
}

struct StackTracker {
    stacks: Vec<VecDeque<char>>,
    commands: Vec<Command>,
}

impl StackTracker {
    const NUM_STACKS: usize = 9;

    fn new(file_path: &str) -> Self {
        let mut dividing_line = String::from("");
        let mut stacks = Vec::with_capacity(Self::NUM_STACKS);
        for n in 1..=Self::NUM_STACKS {
            let end_chars = if n == Self::NUM_STACKS { "\n\n" } else { " " };
            dividing_line.push_str(&format!(" {} {}", n, end_chars));
            stacks.push(VecDeque::new());
        }

        let file_content = fs::read_to_string(file_path).unwrap();
        let mut split_file = file_content.split(&dividing_line);
        let (unparsed_crates, unparsed_commands) =
            (split_file.next().unwrap(), split_file.next().unwrap());

        for unparsed_crate_row in unparsed_crates.lines() {
            for (i, char) in unparsed_crate_row.chars().enumerate() {
                if char != ' ' && char != '[' && char != ']' {
                    let stack_index = i / 4; // integer division
                    stacks[stack_index].push_back(char);
                }
            }
        }

        let mut commands = Vec::new();
        for unparsed_command in unparsed_commands.lines() {
            commands.push(Command::new(unparsed_command));
        }

        Self { stacks, commands }
    }

    fn execute_all(&mut self) {
        let commands = self.commands.clone();
        commands
            .iter()
            .cloned()
            .for_each(|command| self.execute(command))
    }

    fn execute_all_multi_crate_move(&mut self) {
        let commands = self.commands.clone();
        commands
            .iter()
            .cloned()
            .for_each(|command| self.execute_multi_crate_move(command))
    }

    fn execute(&mut self, command: Command) {
        for _ in 0..command.num_crates {
            if let Some(removed_crate) = self.stacks[command.source_stack_index].pop_front() {
                self.stacks[command.dest_stack_index].push_front(removed_crate);
            }
        }
    }

    fn execute_multi_crate_move(&mut self, command: Command) {
        let mut intermediate = VecDeque::new();
        for _ in 0..command.num_crates {
            if let Some(removed_crate) = self.stacks[command.source_stack_index].pop_front() {
                intermediate.push_front(removed_crate);
            }
        }
        for moved_crate in intermediate.into_iter() {
            self.stacks[command.dest_stack_index].push_front(moved_crate);
        }
    }

    fn top_chars(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.front().unwrap())
            .collect::<String>()
    }
}

// find away not to rebuild regex on every call of Command::new
// any way to avoid cloning commands?
