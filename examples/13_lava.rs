use std::fs;

fn parse_input(path: &str) -> Vec<Vec<Vec<char>>> {
    let mut res = Vec::new();
    let file = fs::read_to_string(path).expect("cannot read file");
    let lines = file.lines();
    let mut scenario = Vec::new();
    for line in lines {
        if line.is_empty() {
            res.push(scenario);
            scenario = Vec::new();
        } else {
            scenario.push(line.chars().collect());
        }
    }
    res.push(scenario);
    res
}

fn exercise1(scenario: &[Vec<Vec<char>>]) -> usize {
    let mut res = 0;
    for input in scenario {
        // find horizontal pattern
        let mut i = 1;
        while i < input.len() {
            let mut matches = 0;
            let max_offset = std::cmp::min(i, input.len() - i);
            for offset in 0..max_offset {
                if input[i - 1 - offset] == input[i + offset] {
                    matches += 1;
                }
            }
            if matches == max_offset {
                println!("found horizontal pattern at row {}", i);
                res += 100 * i;
            }
            i += 1;
        }
        // find vertical pattern
        let mut i = 1;
        while i < input[0].len() {
            let mut matches = 0;
            let max_offset = std::cmp::min(i, input[0].len() - i);
            for offset in 0..max_offset {
                let mut match_line = true;
                for line in input {
                    if line[i - 1 - offset] != line[i + offset] {
                        match_line = false;
                        break;
                    }
                }
                if match_line {
                    matches += 1;
                }
            }
            if matches == max_offset {
                println!("found vertical pattern at column {}", i);
                res += i;
            }
            i += 1;
        }
    }
    res
}

fn exercise2(scenario: &[Vec<Vec<char>>]) -> usize {
    let mut res = 0;
    for input in scenario {
        // find horizontal pattern
        let mut i = 1;
        let mut found = false;
        while i < input.len() {
            let mut diff = 0;
            let max_offset = std::cmp::min(i, input.len() - i);
            for offset in 0..max_offset {
                let a = &input[i - 1 - offset];
                let b = &input[i + offset];
                for j in 0..a.len() {
                    if a[j] != b[j] {
                        diff += 1;
                        if diff > 1 {
                            break;
                        }
                    }
                }
            }
            if diff == 1 {
                println!("found horizontal pattern at row {}", i);
                res += 100 * i;
                found = true;
                break;
            }
            i += 1;
        }
        if found {
            continue;
        }
        // find vertical pattern
        let mut i = 1;
        while i < input[0].len() {
            let mut diff = 0;
            let max_offset = std::cmp::min(i, input[0].len() - i);
            for offset in 0..max_offset {
                for line in input {
                    if line[i - 1 - offset] != line[i + offset] {
                        diff += 1;
                        if diff > 1 {
                            break;
                        }
                    }
                }
            }
            if diff == 1 {
                println!("found vertical pattern at column {}", i);
                res += i;
                found = true;
                break;
            }
            i += 1;
        }
        assert!(found);
    }
    res
}

fn main() {
    let input = parse_input("data/13_input.txt");
    println!("exercise 1: {}", exercise1(&input));
    println!("exercise 2: {}", exercise2(&input));
}
