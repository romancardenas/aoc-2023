// NOT MINE: FROM https://github.com/sopyb/AoC/blob/main/2023/day_12/src/combined.rs

use std::collections::HashMap;
use std::fs;

fn calculate_solutions(
    characters: &[char],
    integers: &[u128],
    memoization: &mut HashMap<(Vec<u128>, Vec<char>), u128>,
) -> u128 {
    if characters.is_empty() {
        if integers.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    match characters[0] {
        '.' => calculate_solutions(&characters[1..], integers, memoization),
        '#' => calculate_hash_solutions(integers, characters, memoization),
        '?' => {
            calculate_solutions(&characters[1..], integers, memoization)
                + calculate_hash_solutions(integers, characters, memoization)
        }
        _ => panic!(">.> WHAT DID YOU DO?"),
    }
}

fn calculate_hash_solutions(
    integers: &[u128],
    characters: &[char],
    memoization: &mut HashMap<(Vec<u128>, Vec<char>), u128>,
) -> u128 {
    if let Some(&result) = memoization.get(&(integers.to_vec(), characters.to_vec())) {
        return result;
    }

    if integers.is_empty() {
        return 0;
    }

    let x = integers[0] as usize;
    if characters.len() < x {
        return 0;
    }
    for c in &characters[..x] {
        if *c == '.' {
            return 0;
        }
    }
    if characters.len() == x {
        if integers.len() == 1 {
            return 1;
        }
        return 0;
    }
    if characters[x] == '#' {
        return 0;
    }
    let result = calculate_solutions(&characters[(x + 1)..], &integers[1..], memoization);
    memoization.insert((integers.to_vec(), characters.to_vec()), result);
    result
}

fn main() {
    let input = fs::read_to_string("data/12_input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut data_rows = Vec::new();
    let mut memoization = HashMap::new();

    for line in &lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let springs = parts[0].chars().collect::<Vec<_>>();
        let groups: Vec<u128> = parts[1].split(',').map(|s| s.parse().unwrap()).collect();
        data_rows.push((springs, groups));
    }

    let total: u128 = data_rows
        .iter()
        .map(|(springs, groups)| calculate_solutions(springs, groups, &mut memoization))
        .sum();

    println!("Part1: {}", total);

    data_rows.clear();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let springs: Vec<&str> = parts[0].split('?').collect();
        let groups: Vec<&str> = parts[1].split(',').collect();

        let new_springs: String = springs
            .iter()
            .cycle()
            .take(springs.len() * 5)
            .cloned()
            .collect::<Vec<&str>>()
            .join("?");
        let new_groups: String = groups
            .iter()
            .cycle()
            .take(groups.len() * 5)
            .cloned()
            .collect::<Vec<&str>>()
            .join(",");

        let springs_chars: Vec<char> = new_springs.chars().collect();
        let groups_int: Vec<u128> = new_groups.split(',').map(|s| s.parse().unwrap()).collect();

        data_rows.push((springs_chars, groups_int));
    }

    let total: u128 = data_rows
        .iter()
        .map(|(springs, groups)| calculate_solutions(springs, groups, &mut memoization))
        .sum();
    println!("Part2: {}", total);
}
