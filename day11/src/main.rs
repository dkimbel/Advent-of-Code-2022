use regex::Regex;
use std::collections::VecDeque;
use std::fs;

fn main() {
    let mut simulator = MonkeySimulator::new("resources/input_1");
    let solution_1 = simulator.simulate(20, Some(|worry| worry / 3));
    println!("Part 1 solution: {}", solution_1);

    let mut simulator_2 = MonkeySimulator::new("resources/input_1");
    let solution_2 = simulator_2.simulate(10000, None);
    println!("Part 2 solution: {}", solution_2);
}

#[derive(Clone)]
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

#[derive(Clone)]
enum OpValue {
    Old,
    Num(u64),
}

impl OpValue {
    fn new(str: &str) -> Self {
        if str == "old" {
            Self::Old
        } else {
            let parsed = str.parse::<u64>().unwrap();
            Self::Num(parsed)
        }
    }
}

#[derive(Clone)]
struct Operation {
    operand: Operand,
    value: OpValue,
}

impl Operation {
    fn evaluate(&self, input: u64) -> u64 {
        match (&self.operand, &self.value) {
            (Operand::Add, OpValue::Old) => input + input,
            (Operand::Add, OpValue::Num(num)) => input + num,
            (Operand::Multiply, OpValue::Old) => input * input,
            (Operand::Multiply, OpValue::Num(num)) => input * num,
        }
    }
}

#[derive(Clone)]
struct Target {
    divisor: u64,
    true_monkey_index: usize,
    false_monkey_index: usize,
}

impl Target {
    fn evaluate(&self, input: u64) -> usize {
        if input % self.divisor == 0 {
            self.true_monkey_index
        } else {
            self.false_monkey_index
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
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
            r"^Monkey \d+:\n {2}Starting items: ([\d, ]+)\n {2}Operation: new = old ([+*]) (old|\d+)\n {2}Test: divisible by (\d+)\n {4}If true: throw to monkey (\d+)\n {4}If false: throw to monkey (\d+)$",
        )
        .unwrap();

        let mut monkeys = Vec::new();

        for unparsed in unparsed_monkeys {
            let cap = re.captures(unparsed).unwrap();
            let items: VecDeque<u64> = cap[1]
                .split(", ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<VecDeque<_>>();
            let operation = Operation {
                operand: Operand::new(cap[2].parse::<char>().unwrap()),
                value: OpValue::new(&cap[3]),
            };
            let target = Target {
                divisor: cap[4].parse::<u64>().unwrap(),
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

    // admittedly naive
    fn greatest_common_factor(&self) -> u64 {
        let monkey_divisors: Vec<u64> = self
            .monkeys
            .iter()
            .map(|monkey| monkey.target.divisor)
            .collect();
        let all_multiplied = monkey_divisors.iter().product();
        for n in 2..=all_multiplied {
            if monkey_divisors.iter().all(|divisor| n % divisor == 0) {
                return n;
            }
        }
        panic!("Failed to find max common factor across monkeys!")
    }

    fn simulate(&mut self, num_rounds: u32, relief_fn: Option<fn(u64) -> u64>) -> u32 {
        let greatest_common_factor = self.greatest_common_factor();

        for _ in 1..=num_rounds {
            for current_monkey_index in 0..self.monkeys.len() {
                let mut monkey = self.monkeys[current_monkey_index].clone();
                while let Some(item) = monkey.items.pop_front() {
                    let item = item % greatest_common_factor;
                    let after_eval = monkey.operation.evaluate(item);
                    monkey.num_items_inspected += 1;
                    let after_relief = match relief_fn {
                        Some(func) => func(after_eval),
                        None => after_eval,
                    };
                    let target_monkey_index = monkey.target.evaluate(after_relief);
                    let target_monkey = if current_monkey_index == target_monkey_index {
                        &mut monkey
                    } else {
                        self.monkeys.get_mut(target_monkey_index).unwrap()
                    };
                    target_monkey.items.push_back(after_relief);
                }
                // overwrite our main mutable vec's monkey with the copy whose items we just emptied
                self.monkeys[current_monkey_index] = monkey;
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
        nums_inspected.iter().take(2).product()
    }
}
