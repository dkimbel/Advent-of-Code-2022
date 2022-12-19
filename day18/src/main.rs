use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("resources/input_1").unwrap();
    let reader = BufReader::new(file);

    let mut occupied_coords = HashSet::new();
    let mut surface_area = 0;

    for line in reader.lines() {
        let line_content = line.unwrap();
        let split = line_content
            .splitn(3, ',')
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let (x, y, z) = (split[0], split[1], split[2]);

        let num_occupied_adjacents = occupied_coords
            .intersection(&HashSet::from([
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z + 1),
                (x, y, z - 1),
            ]))
            .collect::<Vec<_>>()
            .len();
        let surface_area_diff = 6 - (num_occupied_adjacents as isize * 2);

        surface_area += surface_area_diff;
        occupied_coords.insert((x, y, z));
    }

    println!("Part 1 solution: {}", surface_area);
}
