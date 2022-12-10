use std::collections::HashMap;
use std::fs::File as StdFile;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn main() {
    let analyzer = FilesystemAnalyzer::new("resources/input_1");
    let directory_sizes = analyzer.directory_sizes();
    let solution_1: u32 = directory_sizes
        .values()
        .filter(|&&size| size <= 100_000)
        .sum();
    println!("Part 1 solution: {}", solution_1);

    let space_used = directory_sizes.get("").unwrap();
    let space_remaining = 70_000_000 - space_used;
    let space_needed = 30_000_000 - space_remaining;
    let mut large_enough_sizes = directory_sizes
        .values()
        .filter(|&&size| size >= space_needed)
        .collect::<Vec<_>>();
    large_enough_sizes.sort();
    let solution_2 = large_enough_sizes[0];
    println!("Part 2 solution: {}", solution_2);
}

#[derive(Debug)]
enum Command {
    ListObjects,
    IntoDirectory { directory_name: String },
    BackDirectory,
}

#[derive(Clone, Debug)]
enum FilesystemObject {
    File(File),
    Directory(Directory),
}

impl FilesystemObject {
    fn name(&self) -> String {
        match self {
            Self::File(File { name, .. }) => name.clone(),
            Self::Directory(Directory { name, .. }) => name.clone(),
        }
    }
}

#[derive(Clone, Debug)]
struct File {
    name: String,
    size: u32,
}

#[derive(Clone, Debug)]
struct Directory {
    name: String,
}

impl Directory {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

#[derive(Debug)]
enum Input {
    Command(Command),
    FilesystemObject(FilesystemObject),
}

struct FilesystemAnalyzer {
    inputs: Vec<Input>,
}

impl FilesystemAnalyzer {
    fn new(file_path: &str) -> Self {
        let file = StdFile::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let into_directory_regex = Regex::new(r"^\$ cd ([\w|/]+)$").unwrap();
        let directory_regex = Regex::new(r"dir ([\w|/]+)$").unwrap();
        let file_regex = Regex::new(r"^(\d+) ([\w|.]+)$").unwrap();

        let mut inputs = Vec::new();
        for line in reader.lines() {
            let line_content = &line.unwrap();

            let input = if line_content == "$ ls" {
                Input::Command(Command::ListObjects)
            } else if line_content == "$ cd .." {
                Input::Command(Command::BackDirectory)
            } else if let Some(captures) = into_directory_regex.captures(line_content) {
                Input::Command(Command::IntoDirectory {
                    directory_name: String::from(&captures[1]),
                })
            } else if let Some(captures) = directory_regex.captures(line_content) {
                Input::FilesystemObject(FilesystemObject::Directory(Directory::new(&captures[1])))
            } else if let Some(captures) = file_regex.captures(line_content) {
                Input::FilesystemObject(FilesystemObject::File(File {
                    name: String::from(&captures[2]),
                    size: captures[1].parse::<u32>().unwrap(),
                }))
            } else {
                panic!("Unable to parse input line {}", line_content)
            };

            inputs.push(input);
        }

        Self { inputs }
    }

    fn directory_sizes(&self) -> HashMap<String, u32> {
        let mut current_path: Vec<&str> = Vec::new();
        let mut directory_sizes: HashMap<String, u32> = HashMap::new();

        // TODO just track current path and update size of EVERY relevant dir as you go
        for input in self.inputs.iter() {
            match input {
                Input::FilesystemObject(FilesystemObject::File(file)) => {
                    // increase running size of all directories in path; if they
                    // don't exist yet, add key for them
                    for n in 1..=current_path.len() {
                        let path = current_path
                            .iter()
                            .take(n)
                            .cloned()
                            .collect::<Vec<_>>()
                            .join("/");
                        let directory_size = directory_sizes.entry(path).or_insert(0);
                        *directory_size += file.size;
                    }
                }
                Input::Command(Command::BackDirectory) => {
                    current_path.pop();
                }
                Input::Command(Command::IntoDirectory { directory_name }) => {
                    let parsed_name = if directory_name == "/" {
                        // give root directory a special name that makes it less
                        // cumbersome to join, and maybe split, on `/`
                        ""
                    } else {
                        directory_name
                    };
                    current_path.push(parsed_name);
                }
                _ => (),
            }
        }

        directory_sizes
    }
}

// attempt to reduce uses of clone
