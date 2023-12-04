use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{prelude::*, BufReader},
};

fn parse_input(path: &str) -> Vec<(HashSet<u32>, HashSet<u32>)> {
    let mut res = Vec::new();

    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    for line in reader.lines().map_while(Result::ok) {
        let line = line.split(':').last().unwrap();
        let numbers = line.split('|').map(|s| s.trim()).collect::<Vec<_>>();

        let winning: HashSet<u32> = numbers[0]
            .split(' ')
            .map(|n| n.parse::<u32>())
            .filter_map(Result::ok)
            .collect();
        let mine: HashSet<u32> = numbers[1]
            .split(' ')
            .map(|n| n.parse::<u32>())
            .filter_map(Result::ok)
            .collect();
        res.push((winning, mine));
    }
    res
}

fn exercise_1(games: &[(HashSet<u32>, HashSet<u32>)]) -> u32 {
    let mut res = 0;
    for (winning, mine) in games {
        let mut count = 0;
        for n in mine {
            if winning.contains(n) {
                count += 1;
            }
        }
        if count != 0 {
            res += 2u32.pow(count - 1);
        }
    }
    res
}

fn exercise_2(games: &[(HashSet<u32>, HashSet<u32>)]) -> u32 {
    let mut res = 0;

    // Initially, all the games have one card
    let mut pending = HashMap::new();
    for i in 0..games.len() {
        let n_cards = *pending.entry(i).or_insert(1);
        res += n_cards;
        // count winning numbers
        let (winning, mine) = &games[i];
        let mut count = 0;
        for n in mine {
            if winning.contains(n) {
                count += 1;
            }
        }
        println!("game {}: {} maches, {} copies", i + 1, count, n_cards);

        for j in 1..=count {
            let k = i + j;
            if k < games.len() {
                let n = pending.entry(k).or_insert(1);
                *n += n_cards;
            }
        }
    }
    res
}

fn main() {
    let input = parse_input("data/4_input.txt");
    println!("exercise 1: {}", exercise_1(&input));

    println!("exercise 2: {}", exercise_2(&input));
}
