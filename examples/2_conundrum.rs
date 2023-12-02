use std::fs::File;
use std::io::{prelude::*, BufReader};

fn parse_input(path: &str) -> Vec<Vec<(u32, u32, u32)>> {
    let mut games = Vec::new();
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    for line in reader.lines().map_while(Result::ok) {
        let mut game = Vec::new();
        let line = line.split(':').last().unwrap();
        for reveal in line.split(';') {
            let (mut red, mut green, mut blue) = (0, 0, 0);
            for color in reveal.split(',').map(|s| s.trim()) {
                let color = color.split(' ').collect::<Vec<_>>();
                let n = color[0].parse::<u32>().unwrap();
                match color[1] {
                    "red" => red = n,
                    "green" => green = n,
                    "blue" => blue = n,
                    _ => panic!("unknown color"),
                }
            }
            game.push((red, green, blue));
        }
        games.push(game);
    }
    games
}

fn solve_1(games: &[Vec<(u32, u32, u32)>], max_red: u32, max_green: u32, max_blue: u32) -> u32 {
    let mut res = 0;
    for (i, game) in games.iter().enumerate() {
        let mut id = i + 1;
        for (red, green, blue) in game {
            if *red > max_red || *green > max_green || *blue > max_blue {
                id = 0;
                break;
            }
        }
        res += id;
    }
    res as _
}

fn solve_2(games: &Vec<Vec<(u32, u32, u32)>>) -> u32 {
    let mut res = Vec::new();
    for game in games {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for (red, green, blue) in game {
            if *red > max_red {
                max_red = *red;
            }
            if *green > max_green {
                max_green = *green;
            }
            if *blue > max_blue {
                max_blue = *blue;
            }
        }
        res.push(max_red * max_green * max_blue);
    }
    res.iter().sum()
}

fn main() {
    let input = parse_input("data/2_input.txt");
    let res = solve_1(&input, 12, 13, 14);
    println!("res 1: {}", res);
    let res = solve_2(&input);
    println!("res 2: {}", res);
}
