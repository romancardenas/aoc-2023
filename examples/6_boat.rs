use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Scenario {
    time: Vec<usize>,
    distance: Vec<usize>,
}

impl FromStr for Scenario {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vecs = Vec::new();
        let mut lines = s.lines();

        for _ in 0..2 {
            let vec = lines
                .next()
                .ok_or("missing field")?
                .split(':')
                .last()
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap() as usize)
                .collect::<Vec<_>>();
            vecs.push(vec);
        }

        Ok(Self {
            time: vecs[0].clone(),
            distance: vecs[1].clone(),
        })
    }
}

fn parse_input(path: &str) -> Scenario {
    std::fs::read_to_string(path)
        .expect("cannot read file")
        .parse()
        .expect("cannot parse input")
}

fn exercise1(scenario: &Scenario) -> usize {
    let mut ways = Vec::new();

    for i in 0..scenario.time.len() {
        let time = scenario.time[i];
        let max_distance = scenario.distance[i];

        let mut n_ways = 0;
        for speed in 1..time {
            let t = time - speed;
            let distance = speed * t;
            if distance > max_distance {
                n_ways += 1;
            }
        }
        ways.push(n_ways);
    }
    println!("{:?}", ways);

    let mut res = 1;
    for n in ways {
        res *= n;
    }
    res
}

fn exercise2() -> usize {
    let time = 71_530;
    let max_distance = 940_200;

    let time = 46_689_866;
    let max_distance = 358_105_418_071_080;

    let mut min_speed = 0;

    let mut delta = time / 2;
    let mut speed = delta;
    while delta > 0 {
        let t = time - speed;
        let d = speed * t;
        if d > max_distance {
            min_speed = speed;
            speed -= delta;
        } else {
            speed += delta;
        }
        delta /= 2;
    }

    time - 2 * (min_speed - 1) + 1
}

fn main() {
    let scenario = parse_input("data/6_input.txt");
    println!("exercise 1: {}", exercise1(&scenario));
    println!("exercise 2: {}", exercise2());
}
