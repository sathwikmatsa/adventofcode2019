use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn enumerate_path(path: &Vec<&str>) -> HashMap<(i32, i32), u32> {
    let mut coords: HashMap<(i32, i32), u32> = HashMap::new();
    let mut i: i32 = 0;
    let mut j: i32 = 0;

    let mut length: u32 = 0;
    for long_step in path {
        let mini_steps: u32 = long_step[1..].parse().unwrap();
        let direction = long_step.chars().nth(0).unwrap();
        for _ in 0..mini_steps {
            if direction == 'R' {
                i = i + 1;
            } else if direction == 'D' {
                j = j - 1;
            } else if direction == 'U' {
                j = j + 1;
            } else {
                i = i - 1;
            }
            length += 1;
            coords.entry((i, j)).or_insert(length);
        }
    }
    coords
}

fn crossover_manhattan_distance(
    path1: &HashMap<(i32, i32), u32>,
    path2: &HashMap<(i32, i32), u32>,
) -> (u32, u32) {
    let mut shortest_distance_from_center: u32 = std::u32::MAX;
    let mut min_steps_for_cross: u32 = std::u32::MAX;

    for coord in path1
        .keys()
        .collect::<HashSet<_>>()
        .intersection(&path2.keys().collect::<HashSet<_>>())
    {
        let manhattan_distance = (coord.0.abs() + coord.1.abs()) as u32;
        let total_steps = path1[&coord] + path2[&coord];

        if manhattan_distance < shortest_distance_from_center {
            shortest_distance_from_center = manhattan_distance;
        }
        if total_steps < min_steps_for_cross {
            min_steps_for_cross = total_steps;
        }
    }

    (shortest_distance_from_center, min_steps_for_cross)
}

fn main() {
    // get wire paths
    let input = read_to_string("input/03.txt").expect("Unable to read from file: input/03.txt");
    let mut input = input.lines();

    let wire1_path = input.next().unwrap().split(',').collect::<Vec<_>>();
    let wire2_path = input.next().unwrap().split(',').collect::<Vec<_>>();

    let wire1_coords = enumerate_path(&wire1_path);
    let wire2_coords = enumerate_path(&wire2_path);

    let (nearest_to_center, min_steps_to_cross) =
        crossover_manhattan_distance(&wire1_coords, &wire2_coords);

    println!("{}", nearest_to_center);
    println!("{}", min_steps_to_cross);
}
