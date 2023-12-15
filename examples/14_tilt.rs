use std::{collections::HashMap, fs};

fn parse_input(path: &str) -> Vec<Vec<char>> {
    let file = fs::read_to_string(path).expect("cannot read file");
    file.lines().map(|line| line.chars().collect()).collect()
}

fn tilt_north(input: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut res = input.to_vec();
    let mut ceiling = HashMap::new();
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'O' {
                let new_pos = match ceiling.get(&j) {
                    Some(&pos) => pos + 1,
                    None => 0,
                };
                res[i][j] = '.';
                res[new_pos][j] = 'O';
                ceiling.insert(j, new_pos);
            } else if *c == '#' {
                ceiling.insert(j, i);
            }
        }
    }
    res
}

fn tilt_west(input: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut res = input.to_vec();
    let mut ceiling = HashMap::new();
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'O' {
                let new_pos = match ceiling.get(&i) {
                    Some(&pos) => pos + 1,
                    None => 0,
                };
                res[i][j] = '.';
                res[i][new_pos] = 'O';
                ceiling.insert(i, new_pos);
            } else if *c == '#' {
                ceiling.insert(i, j);
            }
        }
    }
    res
}

fn tilt_south(input: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut res = input.to_vec();
    let mut ceiling = HashMap::new();
    let n_rows: usize = input.len();
    for (i, line) in input.iter().enumerate().rev() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'O' {
                let new_pos = match ceiling.get(&j) {
                    Some(&pos) => pos - 1,
                    None => n_rows - 1,
                };
                res[i][j] = '.';
                res[new_pos][j] = 'O';
                ceiling.insert(j, new_pos);
            } else if *c == '#' {
                ceiling.insert(j, i);
            }
        }
    }
    res
}

fn tilt_east(input: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut res = input.to_vec();
    let mut ceiling = HashMap::new();
    let n_cols: usize = input[0].len();
    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.iter().enumerate().rev() {
            if *c == 'O' {
                let new_pos = match ceiling.get(&i) {
                    Some(&pos) => pos - 1,
                    None => n_cols - 1,
                };
                res[i][j] = '.';
                res[i][new_pos] = 'O';
                ceiling.insert(i, new_pos);
            } else if *c == '#' {
                ceiling.insert(i, j);
            }
        }
    }
    res
}

fn tilt(input: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut res = tilt_north(input);
    res = tilt_west(&res);
    res = tilt_south(&res);
    tilt_east(&res)
}

fn exercise1(input: &[Vec<char>]) -> usize {
    let mut res = 0;
    let n_cols = input[0].len();
    for (i, line) in input.iter().enumerate() {
        for c in line {
            if *c == 'O' {
                res += n_cols - i;
            }
        }
    }
    res
}

fn exercise2(input: &[Vec<char>]) -> usize {
    let mut scenario = input.to_vec();
    let mut cache = HashMap::new();
    let mut reverse_cache = HashMap::new();

    let n_expected_shifts = 1000000000;

    let mut n_shifts = 0;
    loop {
        if cache.contains_key(&scenario) {
            println!("Cache hit!");
            println!("    n_shifts: {}", n_shifts);
            let last_n_shifts = cache.get(&scenario).unwrap();
            println!("    last_n_shifts: {}", last_n_shifts);
            let period = n_shifts - last_n_shifts;
            println!("    period: {}", period);
            let n_loops = (n_expected_shifts - n_shifts) / period;
            println!("    n_loops: {}", n_loops);

            let n_shifts_extra = (n_expected_shifts - n_shifts) % period;
            println!("    n_shifts_extra: {}", n_shifts_extra);
            n_shifts = last_n_shifts + n_shifts_extra;
            break;
        } else {
            cache.insert(scenario.clone(), n_shifts);
            reverse_cache.insert(n_shifts, scenario.clone());
            scenario = tilt(&scenario);
            n_shifts += 1;
        }
    }
    exercise1(reverse_cache.get(&n_shifts).unwrap())
}

fn main() {
    let input = parse_input("data/14_input.txt");
    let res = tilt_north(&input);
    println!("res1: {}", exercise1(&res));

    println!("res2: {}", exercise2(&input));
}
