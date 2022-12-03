use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

fn main() {
    let analyzer = RucksackAnalyzer::new("resources/input_1");
    let solution_1 = analyzer.common_priority_sum();
    println!("Part 1 solution: {}", solution_1);

    let solution_2 = analyzer.group_badge_priority_sum();
    println!("Part 2 solution: {}", solution_2);
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Item {
    name: char,
}

impl Item {
    // can only handle a-z, A-Z
    fn priority(&self) -> u32 {
        let as_base_36 = self.name.to_digit(36).unwrap();
        let priority = as_base_36 - 9;
        if self.name.is_uppercase() {
            priority + 26
        } else {
            priority
        }
    }

    fn from_char(char: char) -> Self {
        Self { name: char }
    }
}

struct Rucksack {
    compartment_1: Vec<Item>,
    compartment_2: Vec<Item>,
}

impl Rucksack {
    fn common(&self) -> &Item {
        let compartment_1_set: HashSet<&Item> = HashSet::from_iter(self.compartment_1.iter());
        let compartment_2_set: HashSet<&Item> = HashSet::from_iter(self.compartment_2.iter());

        // a guarantee from the problem is that there will always be one common item
        let common_items = compartment_1_set
            .intersection(&compartment_2_set)
            .map(|&item| item)
            .collect::<Vec<&Item>>();

        common_items[0]
    }
}

struct RucksackAnalyzer {
    rucksacks: Vec<Rucksack>,
}

impl RucksackAnalyzer {
    fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let mut rucksacks = Vec::new();
        for line in reader.lines() {
            let line_content = line.unwrap();
            let line_len = line_content.len();
            let chars = line_content.chars();
            let first_half = chars
                .clone()
                .take(line_len / 2)
                .map(|char| Item::from_char(char))
                .collect::<Vec<_>>();
            let second_half = chars
                .skip(line_len / 2)
                .take(line_len / 2)
                .map(|char| Item::from_char(char))
                .collect::<Vec<_>>();
            rucksacks.push(Rucksack {
                compartment_1: first_half.clone(),
                compartment_2: second_half.clone(),
            });
        }

        Self { rucksacks }
    }

    fn common_priority_sum(&self) -> u32 {
        self.rucksacks
            .iter()
            .map(|rucksack| rucksack.common().priority())
            .sum()
    }

    fn group_badge_priority_sum(&self) -> u32 {
        let mut running_priority = 0;
        let mut rucksack_triad = Vec::new();
        self.rucksacks.iter().for_each(|rucksack| {
            rucksack_triad.push(rucksack);

            if rucksack_triad.len() == 3 {
                let rucksack_content_sets: Vec<HashSet<Item>> = rucksack_triad
                    .iter()
                    .map(|&rucksack| {
                        let mut combined = rucksack.compartment_1.clone();
                        combined.extend(rucksack.compartment_2.clone());
                        HashSet::from_iter(combined)
                    })
                    .collect::<Vec<HashSet<Item>>>();

                let first = &rucksack_content_sets[0];
                let second = &rucksack_content_sets[1];
                let third = &rucksack_content_sets[2];
                let first_second = first
                    .intersection(second)
                    .map(|item| item.clone())
                    .collect::<HashSet<_>>();
                let full_inter = first_second.intersection(third).collect::<Vec<_>>();
                let common_item = full_inter[0];
                running_priority += common_item.priority();
                rucksack_triad.clear();
            }
        });

        running_priority
    }
}

// fix clippy warning about explicit closure for copying elements
// try to reduce use of clone
// try to get rid of very weird map I needed to add an extra &
// generally see more idiomatic solutions, there has to be a cleaner way to wrangle these sets
