use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl From<char> for Pipe {
    fn from(c: char) -> Self {
        match c {
            '|' => Self::NorthSouth,
            '-' => Self::EastWest,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("invalid pipe"),
        }
    }
}

impl std::fmt::Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::NorthSouth => '|',
            Self::EastWest => '-',
            Self::NorthEast => 'L',
            Self::NorthWest => 'J',
            Self::SouthWest => '7',
            Self::SouthEast => 'F',
            Self::Ground => '.',
            Self::Start => 'S',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Scenario(Vec<Vec<Pipe>>);

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
    type Target = Vec<Vec<Pipe>>;

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

fn exercise1(scenario: &mut Scenario) -> isize {
    // find start
    let mut start = None;
    for (row, line) in scenario.iter().enumerate() {
        for (col, pipe) in line.iter().enumerate() {
            if *pipe == Pipe::Start {
                start = Some((row, col));
                break;
            }
        }
        if start.is_some() {
            break;
        }
    }
    let start = start.expect("no start found");
    let mut paths = HashMap::new();
    let mut pendings = vec![(start, 0isize)];

    while let Some((point, n_steps)) = pendings.pop() {
        // check if new point is a better path. If not, skip it
        if let Some(&min_steps) = paths.get(&point) {
            if min_steps < n_steps {
                continue;
            }
        }
        paths.insert(point, n_steps);

        let pipe = scenario[point.0][point.1];

        // check if pipe can go north
        if point.0 > 0
            && (pipe == Pipe::Start
                || pipe == Pipe::NorthSouth
                || pipe == Pipe::NorthEast
                || pipe == Pipe::NorthWest)
        {
            let north = (point.0 - 1, point.1);
            let north_pipe = scenario[north.0][north.1];
            // check if north pip can go south
            if north_pipe == Pipe::NorthSouth
                || north_pipe == Pipe::SouthEast
                || north_pipe == Pipe::SouthWest
            {
                pendings.push((north, n_steps + 1));
            }
        }

        // check if pipe can go south
        if point.0 < scenario.len() - 1
            && (pipe == Pipe::Start
                || pipe == Pipe::NorthSouth
                || pipe == Pipe::SouthEast
                || pipe == Pipe::SouthWest)
        {
            let south = (point.0 + 1, point.1);
            let south_pipe = scenario[south.0][south.1];
            // check if south pip can go north
            if south_pipe == Pipe::NorthSouth
                || south_pipe == Pipe::NorthEast
                || south_pipe == Pipe::NorthWest
            {
                pendings.push((south, n_steps + 1));
            }
        }

        // check if pipe can go east
        if point.1 < scenario[point.0].len() - 1
            && (pipe == Pipe::Start
                || pipe == Pipe::EastWest
                || pipe == Pipe::NorthEast
                || pipe == Pipe::SouthEast)
        {
            let east = (point.0, point.1 + 1);
            let east_pipe = scenario[east.0][east.1];
            // check if east pip can go west
            if east_pipe == Pipe::EastWest
                || east_pipe == Pipe::NorthWest
                || east_pipe == Pipe::SouthWest
            {
                pendings.push((east, n_steps + 1));
            }
        }

        // check if pipe can go west
        if point.1 > 0
            && (pipe == Pipe::Start
                || pipe == Pipe::EastWest
                || pipe == Pipe::NorthWest
                || pipe == Pipe::SouthWest)
        {
            let west = (point.0, point.1 - 1);
            let west_pipe = scenario[west.0][west.1];
            // check if west pip can go east
            if west_pipe == Pipe::EastWest
                || west_pipe == Pipe::NorthEast
                || west_pipe == Pipe::SouthEast
            {
                pendings.push((west, n_steps + 1));
            }
        }
    }

    // clean scenario
    for (row, pipe) in scenario.iter_mut().enumerate() {
        for (col, pipe) in pipe.iter_mut().enumerate() {
            if !paths.contains_key(&(row, col)) {
                *pipe = Pipe::Ground;
            }
        }
    }
    // guess start pipe type and set it
    let start_north = (start.0 - 1, start.1);
    let start_south = (start.0 + 1, start.1);
    let start_east = (start.0, start.1 + 1);
    let start_west = (start.0, start.1 - 1);

    if paths.contains_key(&start_north) && paths.contains_key(&start_south) {
        scenario[start.0][start.1] = Pipe::NorthSouth;
    } else if paths.contains_key(&start_east) && paths.contains_key(&start_west) {
        scenario[start.0][start.1] = Pipe::EastWest;
    } else if paths.contains_key(&start_north) && paths.contains_key(&start_east) {
        scenario[start.0][start.1] = Pipe::NorthEast;
    } else if paths.contains_key(&start_north) && paths.contains_key(&start_west) {
        scenario[start.0][start.1] = Pipe::NorthWest;
    } else if paths.contains_key(&start_south) && paths.contains_key(&start_east) {
        scenario[start.0][start.1] = Pipe::SouthEast;
    } else if paths.contains_key(&start_south) && paths.contains_key(&start_west) {
        scenario[start.0][start.1] = Pipe::SouthWest;
    } else {
        panic!("cannot guess start pipe type");
    }

    *paths.values().max().unwrap()
}

fn exercise2(scenario: &Scenario) -> usize {
    // find inside tiles
    let mut candidates = HashSet::new();
    for (row, line) in scenario.iter().enumerate() {
        let mut prev = None;
        for (current, &tile) in line.iter().enumerate() {
            match tile {
                Pipe::NorthSouth | Pipe::NorthEast | Pipe::NorthWest => {
                    prev = if let Some(p_col) = prev {
                        for col in p_col + 1..current {
                            if scenario[row][col] == Pipe::Ground {
                                candidates.insert((row, col));
                            }
                        }
                        None
                    } else {
                        Some(current)
                    };
                }
                _ => {}
            }
        }
        println!();
    }

    // candidates = candidates.difference(&outside).cloned().collect();

    for (row, line) in scenario.iter().enumerate() {
        for (col, &tile) in line.iter().enumerate() {
            if candidates.contains(&(row, col)) {
                assert!(tile == Pipe::Ground);
                print!("I");
            } else if tile != Pipe::Ground {
                print!(" *");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    candidates.len()
}

fn main() {
    let mut scenario = parse_input("data/10_input.txt");
    println!("{}", scenario);
    println!("exercise 1: {}", exercise1(&mut scenario));
    println!("{}", scenario);
    println!("exercise 2: {}", exercise2(&scenario));
}
