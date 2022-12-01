use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let analyzer = CalorieAnalyzer::new("resources/input_1")?;
    let max_calories = analyzer.find_max()?;
    println!(
        "Part 1 solution (max calories carried by any elf): {}",
        max_calories
    );

    let top_three_calories = analyzer.find_top_three();
    println!(
        "Part 2 solution (max calories carried by top three elves): {}",
        top_three_calories
    );
    Ok(())
}

#[derive(Clone)]
struct FoodItem {
    calories: u32,
}

struct Elf {
    food_items: Vec<FoodItem>,
}

impl Elf {
    fn held_calories(&self) -> u32 {
        if self.food_items.is_empty() {
            0
        } else {
            self.food_items.iter().map(|food| food.calories).sum()
        }
    }
}

struct CalorieAnalyzer {
    elves: Vec<Elf>,
}

impl CalorieAnalyzer {
    fn new(file_path: &str) -> Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut elves = Vec::new();
        let mut current_elf_food_items = Vec::new();
        for line in reader.lines() {
            let line_content = &line?;
            if line_content.is_empty() {
                elves.push(Elf {
                    food_items: current_elf_food_items.clone(),
                });
                current_elf_food_items.clear();
            } else {
                let calories: u32 = line_content.parse()?;
                current_elf_food_items.push(FoodItem { calories })
            }
        }

        Ok(Self { elves })
    }

    fn find_max(&self) -> Result<u32> {
        self.elves
            .iter()
            .map(|elf| elf.held_calories())
            .max()
            .context("Failed to find max calories")
    }

    fn find_top_three(&self) -> u32 {
        let mut all_held_calories = self
            .elves
            .iter()
            .map(|elf| elf.held_calories())
            .collect::<Vec<u32>>();

        all_held_calories.sort_by(|a, b| b.cmp(a)); // descending

        all_held_calories.iter().take(3).sum()
    }
}
