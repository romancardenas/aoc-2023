use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Item {
    Empty,
    Galaxy,
}

impl From<char> for Item {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Galaxy,
            _ => panic!("invalid char"),
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Empty => '.',
            Self::Galaxy => '#',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Scenario(Vec<Vec<Item>>);

impl std::fmt::Display for Scenario {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.0.iter() {
            for pipe in line.iter() {
                write!(f, "{}", pipe)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Deref for Scenario {
    type Target = Vec<Vec<Item>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Scenario {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn parse_input(path: &str) -> Scenario {
    let mut res = Vec::new();
    let file = std::fs::read_to_string(path).expect("cannot read file");
    for line in file.lines() {
        let samples = line.chars().map(|s| s.into()).collect::<Vec<_>>();
        res.push(samples);
    }
    Scenario(res)
}

fn expanded_space(scenario: &Scenario) -> (Vec<usize>, Vec<usize>) {
    let (mut rows, mut cols) = (Vec::new(), Vec::new());

    for (row, line) in scenario.iter().enumerate() {
        if line.iter().all(|item| *item == Item::Empty) {
            rows.push(row);
        }
    }
    let mut j = 0;
    while j < scenario[0].len() {
        if scenario.iter().all(|line| line[j] == Item::Empty) {
            cols.push(j);
        }
        j += 1;
    }

    (rows, cols)
}

fn exercise1(
    scenario: &Scenario,
    expanded_rows: &[usize],
    expanded_cols: &[usize],
    expansion: usize,
) -> usize {
    // find all galaxies
    let mut galaxies = Vec::new();
    for (row, line) in scenario.iter().enumerate() {
        for (col, item) in line.iter().enumerate() {
            if *item == Item::Galaxy {
                galaxies.push((row, col));
            }
        }
    }

    let mut res = 0;
    for (n, g1) in galaxies.iter().enumerate() {
        for g2 in galaxies.iter().skip(n + 1) {
            let (min_row, max_row) = (g1.0.min(g2.0), g1.0.max(g2.0));
            let (min_col, max_col) = (g1.1.min(g2.1), g1.1.max(g2.1));

            let mut distance = max_row - min_row + max_col - min_col;
            for expanded_row in expanded_rows.iter() {
                if (min_row..=max_row).contains(expanded_row) {
                    distance += expansion - 1;
                }
            }
            for expanded_col in expanded_cols.iter() {
                if (min_col..=max_col).contains(expanded_col) {
                    distance += expansion - 1;
                }
            }
            res += distance;
        }
    }
    res
}

fn main() {
    let scenario = parse_input("data/11_input.txt");
    let (expanded_rows, expanded_cols) = expanded_space(&scenario);
    println!(
        "exercise1: {}",
        exercise1(&scenario, &expanded_rows, &expanded_cols, 2)
    );
    println!(
        "exercise2: {}",
        exercise1(&scenario, &expanded_rows, &expanded_cols, 1_000_000)
    );
}
