use std::cmp;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let coords = parse_coords("resources/input_1");
    let (full_surface_area, occupied_coords) = solve_part_one(&coords);
    println!("Part 1 solution: {}", full_surface_area);
    let outer_surface_area = solve_part_two(occupied_coords);
    println!("Part 2 solution: {}", outer_surface_area);
}

type Coords = (isize, isize, isize);

// Check surface area from the outside, by effectively sending a single-coords-occuping
// 'particle' around the full outside surface. To do this (possibly naively?), search until
// the particle has visited every space in the cube that it possibly can.
fn solve_part_two(occupied_coords: HashSet<Coords>) -> usize {
    let mut surfaces_discovered = 0;

    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
    let mut min_y = isize::MAX;
    let mut max_y = isize::MIN;
    let mut min_z = isize::MAX;
    let mut max_z = isize::MIN;

    for coords in &occupied_coords {
        let (x, y, z) = (coords.0, coords.1, coords.2);
        min_x = cmp::min(min_x, x);
        max_x = cmp::max(max_x, x);
        min_y = cmp::min(min_y, y);
        max_y = cmp::max(max_y, y);
        min_z = cmp::min(min_z, z);
        max_z = cmp::max(max_z, z);
    }

    // make sure we search all the way around the object (must see ALL its surface)
    min_x -= 1;
    min_y -= 1;
    min_z -= 1;
    max_x += 1;
    max_y += 1;
    max_z += 1;

    let mut visited: HashSet<Coords> = HashSet::new();
    let starting_search_coords = (min_x, min_y, min_z);
    let mut coords_to_search = VecDeque::from([starting_search_coords]);

    while let Some(coords) = coords_to_search.pop_front() {
        if visited.contains(&coords) {
            continue;
        }

        let (x, y, z) = (coords.0, coords.1, coords.2);
        if x > max_x || x < min_x || y > max_y || y < min_y || z > max_z || z < min_z {
            continue;
        }

        // Check the surrounding six tiles. If they're part of the object, account for them;
        // otherwise, add them to our deque so we'll search their surroundings too.
        for adjacent_coords in [
            (x + 1, y, z),
            (x - 1, y, z),
            (x, y + 1, z),
            (x, y - 1, z),
            (x, y, z + 1),
            (x, y, z - 1),
        ] {
            if occupied_coords.contains(&adjacent_coords) {
                surfaces_discovered += 1;
            } else {
                coords_to_search.push_back(adjacent_coords);
            }
        }

        visited.insert(coords);
    }

    surfaces_discovered
}

fn solve_part_one(coords: &[Coords]) -> (isize, HashSet<Coords>) {
    let mut occupied_coords: HashSet<Coords> = HashSet::new();
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

fn parse_coords(file_path: &str) -> Vec<Coords> {
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
