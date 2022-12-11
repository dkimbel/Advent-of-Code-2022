use regex::Regex;
use std::collections::VecDeque;
use std::fs;

fn main() {
    let mut simulator = MonkeySimulator::new("resources/input_1");
    let solution_1 = simulator.simulate(20);
    println!("Part 1 solution: {}", solution_1);
}

enum Operand {
    Multiply,
    Add,
}

impl Operand {
    fn new(char: char) -> Self {
        match char {
            '+' => Self::Add,
            '*' => Self::Multiply,
            _ => panic!("Could not find operand for {}", char),
        }
    }
}

enum OpValue {
    Old,
    Num(u32),
}

impl OpValue {
    fn new(str: &str) -> Self {
        if str == "old" {
            Self::Old
        } else {
            let parsed = str.parse::<u32>().unwrap();
            Self::Num(parsed)
        }
    }
}

struct Operation {
    operand: Operand,
    value: OpValue,
}

impl Operation {
    fn evaluate(&self, input: u32) -> u32 {
        let value = match self.value {
            OpValue::Old => input,
            OpValue::Num(num) => num,
        };
        match self.operand {
            Operand::Add => input + value,
            Operand::Multiply => input * value,
        }
    }
}

struct Target {
    divisor: u32,
    true_monkey_index: usize,
    false_monkey_index: usize,
}

impl Target {
    fn evaluate(&self, input: u32) -> usize {
        if input % self.divisor == 0 {
            self.true_monkey_index
        } else {
            self.false_monkey_index
        }
    }
}

struct Monkey {
    items: VecDeque<u32>,
    operation: Operation,
    target: Target,
    num_items_inspected: u32,
}

struct MonkeySimulator {
    monkeys: Vec<Monkey>,
}

impl MonkeySimulator {
    fn new(file_path: &str) -> Self {
        let file_content = fs::read_to_string(file_path).unwrap();
        let unparsed_monkeys = file_content.split("\n\n");
        let re = Regex::new(
            r"^Monkey \d+:\n  Starting items: (\d|,| )+\n  Operation: new = old (\+|\*) (old|\d+)\n  Test: divisible by (\d+)\n    If true: throw to monkey (\d+)\n    If false: throw to monkey(\d+)$",
        )
        .unwrap();

        let mut monkeys = Vec::new();

        for unparsed in unparsed_monkeys {
            let cap = re.captures(unparsed).unwrap();
            let items: VecDeque<u32> = cap[1]
                .split(", ")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<VecDeque<_>>();
            let operation = Operation {
                operand: Operand::new(cap[2].parse::<char>().unwrap()),
                value: OpValue::new(&cap[3]),
            };
            let target = Target {
                divisor: cap[4].parse::<u32>().unwrap(),
                true_monkey_index: cap[5].parse::<usize>().unwrap(),
                false_monkey_index: cap[6].parse::<usize>().unwrap(),
            };
            monkeys.push(Monkey {
                items,
                operation,
                target,
                num_items_inspected: 0,
            });
        }

        Self { monkeys }
    }

    // TODO fix input parsing
    // TODO deal with borrow checker here
    fn simulate(&mut self, num_rounds: u32) -> u32 {
        for _ in 0..num_rounds {
            for mut monkey in self.monkeys.iter_mut() {
                while let Some(item) = monkey.items.pop_front() {
                    let after_eval = monkey.operation.evaluate(item);
                    let after_relief = after_eval / 3;
                    let target_monkey_index = monkey.target.evaluate(after_relief);
                    let mut target_monkey = self.monkeys.get_mut(target_monkey_index).unwrap();
                    target_monkey.items.push_back(after_relief);
                }
            }
        }
        self.monkey_business()
    }

    fn monkey_business(&self) -> u32 {
        let mut nums_inspected = self
            .monkeys
            .iter()
            .map(|monkey| monkey.num_items_inspected)
            .collect::<Vec<_>>();
        nums_inspected.sort_by(|a, b| b.cmp(a));
        nums_inspected.iter().take(2).sum()
    }
}