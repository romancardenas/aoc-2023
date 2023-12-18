use std::collections::HashSet;
use std::fs;

fn parse_input(path: &str) -> Vec<Vec<char>> {
    let file = fs::read_to_string(path).expect("cannot read file");
    file.lines().map(|line| line.chars().collect()).collect()
}

fn exercise_1(input: &[Vec<char>], initial_pos: (i32, i32), initial_dir: (i32, i32)) -> usize {
    let mut cache = HashSet::new();
    let mut energized = HashSet::new();
    let mut beams = vec![(initial_pos, initial_dir)];
    while let Some((mut pos, mut dir)) = beams.pop() {
        if pos.0 < 0 || pos.0 >= input.len() as i32 || pos.1 < 0 || pos.1 >= input[0].len() as i32 {
            continue;
        }
        if !cache.insert((pos, dir)) {
            continue;
        }
        energized.insert(pos);

        let c = input[pos.0 as usize][pos.1 as usize];
        match c {
            '-' => {
                if dir.0 != 0 {
                    beams.push(((pos.0, pos.1 - 1), (0, -1)));
                    beams.push(((pos.0, pos.1 + 1), (0, 1)));
                } else {
                    pos.0 += dir.0;
                    pos.1 += dir.1;
                    beams.push((pos, dir));
                }
            }
            '|' => {
                if dir.1 != 0 {
                    beams.push(((pos.0 - 1, pos.1), (-1, 0)));
                    beams.push(((pos.0 + 1, pos.1), (1, 0)));
                } else {
                    pos.0 += dir.0;
                    pos.1 += dir.1;
                    beams.push((pos, dir));
                }
            }
            _ => {
                match c {
                    '/' => {
                        dir = (-dir.1, -dir.0);
                    }
                    '\\' => {
                        dir = (dir.1, dir.0);
                    }
                    _ => {}
                };
                pos.0 += dir.0;
                pos.1 += dir.1;
                beams.push((pos, dir));
            }
        }

        // for i in 0..input.len() {
        //     for j in 0..input[0].len() {
        //         if energized.contains(&(i as i32, j as i32)) {
        //             print!("X");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
        // println!();
    }
    energized.len()
}

fn exercise_2(input: &[Vec<char>]) -> usize {
    let n_rows = input.len() as i32;
    let n_cols = input[0].len() as i32;
    let mut best = 0;

    for i in 0..n_rows {
        let ex_1 = exercise_1(input, (i, 0), (0, 1));
        let ex_2 = exercise_1(input, (i, n_cols - 1), (0, -1));
        let ex = ex_1.max(ex_2);
        if ex > best {
            best = ex;
            println!("starting at {:?} {:?}: {}", (i, 0), (0, 1), best)
        }
    }
    for j in 0..n_cols {
        let ex_1 = exercise_1(input, (0, j), (1, 0));
        let ex_2 = exercise_1(input, (n_rows - 1, j), (-1, 0));
        let ex = ex_1.max(ex_2);
        if ex > best {
            best = ex;
            println!("starting at {:?} {:?}: {}", (0, j), (1, 0), best)
        }
    }
    best
}

fn main() {
    let input = parse_input("data/16_input.txt");

    println!("Exercise 1: {}", exercise_1(&input, (0, 0), (0, 1)));
    println!("Exercise 2: {}", exercise_2(&input));
}
