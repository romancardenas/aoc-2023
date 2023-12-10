use aoc_2023::lcm;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Scenario {
    directions: Vec<char>,
    nodes: HashMap<String, (String, String)>,
}

fn parse_input(path: &str) -> Scenario {
    let file = std::fs::read_to_string(path).expect("cannot read file");
    let mut lines = file.lines();
    let directions = lines.next().unwrap().chars().collect::<Vec<_>>();
    let mut nodes = HashMap::new();
    for line in lines.skip(1) {
        let parts = line.split('=').collect::<Vec<_>>();
        let origin = parts[0].trim().to_string();
        let destinations = parts[1]
            .trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>();
        nodes.insert(origin, (destinations[0].clone(), destinations[1].clone()));
    }
    Scenario { directions, nodes }
}

fn exercise1(scenario: &Scenario) -> usize {
    let mut current = "AAA".to_string();
    let destination = "ZZZ".to_string();

    let mut steps = 0;
    while current != destination {
        let (left, right) = scenario.nodes.get(&current).unwrap();
        if scenario.directions[steps % scenario.directions.len()] == 'L' {
            current = left.clone();
        } else {
            current = right.clone();
        }
        steps += 1;
    }
    steps
}

fn exercise2(scenario: &Scenario) -> usize {
    let mut destinations = Vec::new();
    for origin in scenario.nodes.keys().filter(|&s| s.ends_with('A')) {
        println!("origin: {}", origin);
        let mut current = origin.clone();
        let mut n_steps = 0;
        while !current.ends_with('Z') {
            let direction = scenario.directions[n_steps % scenario.directions.len()];
            let (left, right) = scenario.nodes.get(&current).unwrap();
            current = if direction == 'L' {
                left.clone()
            } else {
                right.clone()
            };
            n_steps += 1;
        }
        println!("    destination: {} ({} steps)", current, n_steps);
        destinations.push(n_steps);
    }
    println!("destinations: {:?}", destinations);

    destinations.into_iter().fold(1, lcm)
}

fn main() {
    let scenario = parse_input("data/8_input.txt");
    println!("exercise1: {}", exercise1(&scenario));
    println!("exercise2: {}", exercise2(&scenario));
}
