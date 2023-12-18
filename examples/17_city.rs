use std::collections::{BinaryHeap, HashMap};
use std::fs;

fn parse_input(path: &str) -> Vec<Vec<i32>> {
    let file = fs::read_to_string(path).expect("cannot read file");
    file.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn dijkstra(
    start: (i32, i32),
    end: (i32, i32),
    min_step: i32,
    max_step: i32,
    min_turn: i32,
    grid: &[Vec<i32>],
) -> Option<i32> {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();

    let st1 = State {
        point: start,
        dir: (1, 0),
        cost: 0,
    };
    let st2 = State {
        point: start,
        dir: (0, 1),
        cost: 0,
    };

    dist.insert(StateKey::from(st1), 0);
    dist.insert(StateKey::from(st2), 0);
    heap.push(st1);
    heap.push(st2);

    while let Some(state @ State { cost, point, dir }) = heap.pop() {
        if point == end {
            return Some(cost);
        }
        if dist.get(&state.into()).is_some_and(|&c| c < cost) {
            continue;
        }

        let mut new_cost = cost;
        for step in min_step..=max_step {
            let new_point = (point.0 + dir.0 * step, point.1 + dir.1 * step);
            if new_point.0 < 0
                || new_point.0 >= grid.len() as i32
                || new_point.1 < 0
                || new_point.1 >= grid[0].len() as i32
            {
                continue;
            }
            new_cost += grid[new_point.0 as usize][new_point.1 as usize];

            if step < min_turn {
                continue;
            }

            for new_dir in [(-dir.1, -dir.0), (dir.1, dir.0)] {
                let new_state = State {
                    point: new_point,
                    dir: new_dir,
                    cost: new_cost,
                };

                if dist.get(&new_state.into()).is_some_and(|&c| c <= new_cost) {
                    // For p1 and p2
                    // Streak gets too long
                    // Or a better solution/path already exists
                    continue;
                }
                heap.push(new_state);
                dist.insert(new_state.into(), new_state.cost);
            }
        }
    }
    None
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    point: (i32, i32),
    dir: (i32, i32),
    cost: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.point.cmp(&other.point))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct StateKey {
    point: (i32, i32),
    dir: (i32, i32),
}

impl From<State> for StateKey {
    fn from(value: State) -> Self {
        Self {
            point: value.point,
            dir: value.dir,
        }
    }
}

fn main() {
    let loss_matrix = parse_input("data/input.txt");
    let origin = (0, 0);
    let destination = (
        loss_matrix.len() as i32 - 1,
        loss_matrix[0].len() as i32 - 1,
    );

    println!(
        "Exercise 1: {}",
        dijkstra(origin, destination, 1, 3, 1, &loss_matrix).unwrap()
    );

    println!(
        "Exercise 2: {}",
        dijkstra(origin, destination, 1, 10, 4, &loss_matrix).unwrap()
    );
}
