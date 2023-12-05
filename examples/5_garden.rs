use std::{ops::Range, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Transformation {
    destination: Range<usize>,
    source: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Scenario {
    seeds: Vec<usize>,
    stages: Vec<Vec<Transformation>>,
}

impl FromStr for Scenario {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let seeds = lines
            .next()
            .ok_or("missing seeds")?
            .split(':')
            .last()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap() as usize)
            .collect::<Vec<_>>();
        lines.next();

        let mut stages = Vec::new();

        while let Some(_) = lines.next() {
            let mut transformations = Vec::new();
            for line in lines.by_ref() {
                if line.is_empty() {
                    break;
                }
                let vals = line
                    .split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap() as usize)
                    .collect::<Vec<_>>();
                assert_eq!(3, vals.len());
                let destination = vals[0]..vals[0] + vals[2];
                let source = vals[1]..vals[1] + vals[2];
                transformations.push(Transformation {
                    destination,
                    source,
                });
            }
            transformations.sort_by(|a, b| a.destination.start.cmp(&b.destination.start));
            stages.push(transformations);
        }

        Ok(Self { seeds, stages })
    }
}

fn parse_input(path: &str) -> Scenario {
    std::fs::read_to_string(path)
        .expect("cannot read file")
        .parse()
        .expect("cannot parse input")
}

fn exercise_1(scenario: &Scenario) -> usize {
    let mut locations = Vec::new();
    for id in &scenario.seeds {
        let mut seed = *id;
        for stage in &scenario.stages {
            for transformation in stage {
                if transformation.source.contains(&seed) {
                    let distance = seed - transformation.source.start;
                    seed = transformation.destination.start + distance;
                    break;
                }
            }
        }
        locations.push(seed);
    }
    println!();
    locations.into_iter().min().unwrap()
}

fn exercise_2(scenario: &Scenario) -> Option<usize> {
    let mut seeds = Vec::new();
    for i in 0..scenario.seeds.len() / 2 {
        let (from, range) = (scenario.seeds[2 * i], scenario.seeds[2 * i + 1]);
        seeds.push(from..from + range);
    }
    seeds = compress_ranges(seeds);
    println!("Seeds: {:?}", seeds);

    let res = prueba2(&scenario.stages, 0, &seeds);

    res.first().map(|r| r.start)
}

fn compress_ranges(mut ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let mut res = Vec::new();

    let mut i = 0;
    while i < ranges.len() {
        let mut range = ranges[i].clone();
        let mut j = i + 1;
        while j < ranges.len() {
            if ranges[j].start <= range.end {
                range.end = std::cmp::max(range.end, ranges[j].end);
                j += 1;
            } else {
                break;
            }
        }
        res.push(range);
        i = j;
    }
    res
}

fn prueba2(
    stages: &[Vec<Transformation>],
    level: usize,
    origin_ranges: &[Range<usize>],
) -> Vec<Range<usize>> {
    if level >= stages.len() {
        return origin_ranges.to_vec();
    }

    println!("LEVEL {}", level);
    // println!("    Origin ranges: {:?}", origin_ranges);

    // get corresponding stage
    let stage = &stages[level];

    let covered = stage.iter().map(|t| t.source.clone()).collect::<Vec<_>>();
    let covered = compress_ranges(covered);

    let mut uncovered = origin_ranges.to_vec();
    for range in covered {
        let mut i = 0;
        while i < uncovered.len() {
            if uncovered[i].start >= range.start && uncovered[i].end <= range.end {
                // completely covered
                uncovered.remove(i);
            } else if uncovered[i].start >= range.start && uncovered[i].start < range.end {
                // partially covered from the left
                uncovered[i].start = range.end;
                i += 1;
            } else if uncovered[i].end > range.start && uncovered[i].end <= range.end {
                // partially covered from the right
                uncovered[i].end = range.start;
                i += 1;
            } else if uncovered[i].start < range.start && uncovered[i].end > range.end {
                // partially covered in the middle
                uncovered.insert(i + 1, range.end..uncovered[i].end);
                uncovered[i].end = range.start;
                i += 2;
            } else {
                i += 1;
            }
        }
    }
    let mut destination_ranges = compress_ranges(uncovered);
    println!(
        "    Uncovered ranges (propagate as is): {:?}",
        destination_ranges
    );

    // compute potential origin ranges
    for range in origin_ranges {
        for trans in stage.iter() {
            let origin = &trans.source;
            let start = std::cmp::max(origin.start, range.start);
            let end = std::cmp::min(origin.end, range.end);
            // check if range is valid
            if start < end {
                let destination = &trans.destination;
                print!("    Covered range: {}..{} -> ", start, end);
                let start = destination.start + start - origin.start;
                let end = destination.start + end - origin.start;
                println!("{}..{}", start, end);
                destination_ranges.push(start..end);
            }
        }
    }
    destination_ranges.sort_by(|a, b| a.start.cmp(&b.start));
    destination_ranges = compress_ranges(destination_ranges);
    println!("    Destination ranges: {:?}", destination_ranges);

    prueba2(stages, level + 1, &destination_ranges)
}

fn main() {
    let scenario = parse_input("data/5_input.txt");
    println!("Exercise 1: {}", exercise_1(&scenario));
    println!("Exercise 2: {:?}", exercise_2(&scenario));
}
