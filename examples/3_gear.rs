use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: isize,
    col: isize,
}

impl Point {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.row, self.col)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

const NEIGHBOURS: [Point; 8] = [
    Point { row: -1, col: -1 },
    Point { row: -1, col: 0 },
    Point { row: -1, col: 1 },
    Point { row: 0, col: -1 },
    Point { row: 0, col: 1 },
    Point { row: 1, col: -1 },
    Point { row: 1, col: 0 },
    Point { row: 1, col: 1 },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Void,
    Number(u32),
    Symbol(char),
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Field::Void => write!(f, "."),
            Field::Number(n) => write!(f, "{}", n),
            Field::Symbol(c) => write!(f, "{}", c),
        }
    }
}

fn parse_input(path: &str) -> Vec<Vec<Field>> {
    let mut scenario = Vec::new();
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    for line in reader.lines().map_while(Result::ok) {
        let mut row = Vec::new();
        for char in line.chars() {
            let field = match char {
                '.' => Field::Void,
                '0'..='9' => Field::Number(char.to_digit(10).unwrap()),
                _ => Field::Symbol(char),
            };
            row.push(field);
        }
        scenario.push(row);
    }
    scenario
}

fn exercise_1(scenario: &[Vec<Field>]) -> u32 {
    let mut res = 0;
    for i in 0..scenario.len() {
        let mut j = 0;
        while j < scenario[i].len() {
            if let Field::Number(_) = scenario[i][j] {
                let mut number = 0;
                let mut part = false;
                while let Some(Field::Number(n)) = scenario[i].get(j) {
                    let point = Point::new(i as _, j as _);
                    number = number * 10 + n;
                    j += 1;

                    for neigh in NEIGHBOURS.iter() {
                        let neighbour = point + *neigh;
                        if let Some(Field::Symbol(_)) = scenario
                            .get(neighbour.row as usize)
                            .and_then(|row| row.get(neighbour.col as usize))
                        {
                            part = true;
                        }
                    }
                }
                if part {
                    res += number;
                }
            }
            j += 1;
        }
    }
    res
}

fn exercise_2(scenario: &[Vec<Field>]) -> u32 {
    let mut res = HashMap::new();
    for i in 0..scenario.len() {
        let mut j = 0;
        while j < scenario[i].len() {
            if let Field::Number(_) = scenario[i][j] {
                let mut gears = HashSet::new();
                let mut number = 0;
                while let Some(Field::Number(n)) = scenario[i].get(j) {
                    let point = Point::new(i as _, j as _);
                    number = number * 10 + n;
                    j += 1;

                    for neigh in NEIGHBOURS.iter() {
                        let neighbour = point + *neigh;
                        if let Some(Field::Symbol(c)) = scenario
                            .get(neighbour.row as usize)
                            .and_then(|row| row.get(neighbour.col as usize))
                        {
                            if c == &'*' {
                                gears.insert(neighbour);
                            }
                        }
                    }
                }
                for gear in gears {
                    let numbers = res.entry(gear).or_insert(Vec::new());
                    numbers.push(number);
                }
            }
            j += 1;
        }
    }
    res.values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum()
}

fn main() {
    let input = parse_input("data/3_input.txt");

    let res = exercise_1(&input);
    println!("res 1: {}", res);

    let res = exercise_2(&input);
    println!("res 2: {}", res);
}
