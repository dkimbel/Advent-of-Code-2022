use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let analyzer = TreeAnalyzer::new("resources/input_1");
    let num_visible = analyzer.count_visible();
    println!("Part 1 solution: {}", num_visible);

    let scenic = ScenicAnalyzer::new("resources/input_1");
    let max_scenic_score = scenic.max_score();
    println!("Part 2 solution: {}", max_scenic_score)
}

#[derive(Debug)]
struct UnanalyzedTree {
    height: u32,
    visible_left: Option<bool>,
    visible_right: Option<bool>,
    visible_above: Option<bool>,
    visible_below: Option<bool>,
}

impl UnanalyzedTree {
    fn new(height: u32) -> Self {
        Self {
            height,
            visible_left: None,
            visible_below: None,
            visible_above: None,
            visible_right: None,
        }
    }
}

struct Tree {
    height: u32,
    visible: bool,
}

impl Tree {
    fn new(unanalyzed_tree: &UnanalyzedTree) -> Self {
        let visible = unanalyzed_tree.visible_left.unwrap()
            || unanalyzed_tree.visible_right.unwrap()
            || unanalyzed_tree.visible_above.unwrap()
            || unanalyzed_tree.visible_below.unwrap();

        Self {
            height: unanalyzed_tree.height,
            visible,
        }
    }
}

#[derive(Clone)]
struct ScenicTree {
    height: u32,
    scenic_score: Option<u32>,
}

impl ScenicTree {
    fn new(height: u32) -> Self {
        Self {
            height,
            scenic_score: None,
        }
    }
}

struct ScenicAnalyzer {
    trees: Vec<Vec<ScenicTree>>,
}

impl ScenicAnalyzer {
    fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let mut trees = Vec::new();

        for line in reader.lines() {
            let row = line
                .unwrap()
                .chars()
                .map(|char| char.to_digit(10).unwrap())
                .map(ScenicTree::new)
                .collect::<Vec<_>>();
            trees.push(row);
        }

        let copied_trees = trees.clone();

        // operating under the assumption that every row of trees has equal length
        let num_cols = trees[0].len();
        let num_rows = trees.len();

        for row_num in 0..num_rows {
            for col_num in 0..num_cols {
                let mut tree = &mut trees[row_num][col_num];

                let mut num_visible_left = 0;
                let mut left_row_num = row_num;
                while left_row_num > 0 {
                    left_row_num -= 1;
                    num_visible_left += 1;
                    if copied_trees[left_row_num][col_num].height >= tree.height {
                        break;
                    }
                }

                let mut num_visible_right = 0;
                let mut right_row_num = row_num;
                while right_row_num < num_rows - 1 {
                    right_row_num += 1;
                    num_visible_right += 1;
                    if copied_trees[right_row_num][col_num].height >= tree.height {
                        break;
                    }
                }

                let mut num_visible_above = 0;
                let mut above_col_num = col_num;
                while above_col_num > 0 {
                    above_col_num -= 1;
                    num_visible_above += 1;
                    if copied_trees[row_num][above_col_num].height >= tree.height {
                        break;
                    }
                }

                let mut num_visible_below = 0;
                let mut below_col_num = col_num;
                while below_col_num < num_cols - 1 {
                    below_col_num += 1;
                    num_visible_below += 1;
                    if copied_trees[row_num][below_col_num].height >= tree.height {
                        break;
                    }
                }

                tree.scenic_score = Some(
                    num_visible_left * num_visible_right * num_visible_above * num_visible_below,
                )
            }
        }

        Self { trees }
    }

    fn max_score(&self) -> u32 {
        self.trees
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tree| tree.scenic_score.unwrap())
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }
}

struct TreeAnalyzer {
    trees: Vec<Vec<Tree>>,
}

impl TreeAnalyzer {
    fn new(file_path: &str) -> Self {
        let mut unanalyzed_trees = Vec::new();

        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let row = line
                .unwrap()
                .chars()
                .map(|char| char.to_digit(10).unwrap())
                .map(UnanalyzedTree::new)
                .collect::<Vec<_>>();
            unanalyzed_trees.push(row);
        }

        for row in unanalyzed_trees.iter_mut() {
            let mut max_height_left = 0;
            for (i, tree) in row.iter_mut().enumerate() {
                if i == 0 {
                    tree.visible_left = Some(true);
                    max_height_left = tree.height;
                } else {
                    tree.visible_left = Some(tree.height > max_height_left);
                    max_height_left = std::cmp::max(max_height_left, tree.height);
                }
            }
        }

        for row in unanalyzed_trees.iter_mut() {
            let mut max_height_right = 0;
            for (i, tree) in row.iter_mut().rev().enumerate() {
                if i == 0 {
                    tree.visible_right = Some(true);
                    max_height_right = tree.height;
                } else {
                    tree.visible_right = Some(tree.height > max_height_right);
                    max_height_right = std::cmp::max(max_height_right, tree.height);
                }
            }
        }

        // operating under the assumption that every row of trees has equal length
        let num_cols = unanalyzed_trees[0].len();
        let num_rows = unanalyzed_trees.len();

        for col_index in 0..num_cols {
            let mut max_height_above = 0;
            for row_index in 0..num_rows {
                let mut tree = &mut unanalyzed_trees[row_index][col_index];
                if row_index == 0 {
                    tree.visible_above = Some(true);
                    max_height_above = tree.height;
                } else {
                    tree.visible_above = Some(tree.height > max_height_above);
                    max_height_above = std::cmp::max(max_height_above, tree.height);
                }
            }
        }

        for col_index in 0..num_cols {
            let mut max_height_below = 0;
            for row_index in 0..num_rows {
                let reversed_row_index = num_rows - row_index - 1;
                let mut tree = &mut unanalyzed_trees[reversed_row_index][col_index];
                if reversed_row_index == num_rows - 1 {
                    tree.visible_below = Some(true);
                    max_height_below = tree.height;
                } else {
                    tree.visible_below = Some(tree.height > max_height_below);
                    max_height_below = std::cmp::max(max_height_below, tree.height);
                }
            }
        }

        let trees = unanalyzed_trees
            .iter()
            .map(|row| row.iter().map(Tree::new).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { trees }
    }

    fn count_visible(&self) -> u32 {
        self.trees
            .iter()
            .map(|row| row.iter().map(|tree| u32::from(tree.visible)).sum::<u32>())
            .sum()
    }
}

// Check up on Clippy warning about using `loop` instead
// Any way to avoid Clone?
// Note: we could determine the number of visible trees using two fewer full-grid iterations
// if we tracking left and top max heights as we build the initial grid.
// Question: is rev() doing a full extra iteration through the vec? In which case more efficient
// to use indexes to manually work through it in reverse?
