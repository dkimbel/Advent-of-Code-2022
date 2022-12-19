use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let coords = parse_coords("resources/input_1");
    let (full_surface_area, occupied_coords) = solve_part_one(&coords);
    println!("Part 1 solution: {}", full_surface_area);
    solve_part_two(occupied_coords);
}

fn solve_part_two(occupied_coords: HashSet<(isize, isize, isize)>) -> isize {
    todo!()
}

fn solve_part_one(coords: &[(isize, isize, isize)]) -> (isize, HashSet<(isize, isize, isize)>) {
    let mut occupied_coords: HashSet<(isize, isize, isize)> = HashSet::new();
    let mut surface_area = 0;

    for triplet in coords {
        let (x, y, z) = (triplet.0, triplet.1, triplet.2);
        let num_occupied_adjacents = occupied_coords
            .intersection(&HashSet::from([
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z + 1),
                (x, y, z - 1),
            ]))
            .count();
        let surface_area_diff = 6 - (num_occupied_adjacents as isize * 2);

        surface_area += surface_area_diff;
        occupied_coords.insert((x, y, z));
    }

    (surface_area, occupied_coords)
}

fn parse_coords(file_path: &str) -> Vec<(isize, isize, isize)> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut parsed_coords = Vec::new();

    for line in reader.lines() {
        let line_content = line.unwrap();
        let split = line_content
            .splitn(3, ',')
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        parsed_coords.push((split[0], split[1], split[2]));
    }

    parsed_coords
}
