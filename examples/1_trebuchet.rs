use std::fs::File;
use std::io::{prelude::*, BufReader};

/// Reads the input file and returns the list of numbers.
fn read_input_1(path: &str) -> Vec<u32> {
    let mut res = Vec::new();
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    for line in reader.lines().map_while(Result::ok) {
        let (mut first, mut last) = (None, None);
        for char in line.chars() {
            if let Some(n) = char.to_digit(10) {
                if first.is_none() {
                    first = Some(n);
                    last = Some(n);
                } else {
                    last = Some(n);
                }
            }
        }
        res.push(first.unwrap() * 10 + last.unwrap());
    }
    res
}

fn read_input_2(path: &str) -> Vec<u32> {
    let mut res = Vec::new();
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    for line in reader.lines().map_while(Result::ok) {
        let (mut first, mut last) = (None, None);
        for i in 0..line.len() {
            let number = if let Ok(n) = line[i..i + 1].parse::<u32>() {
                Some(n)
            } else if line[i..].starts_with("one") {
                Some(1)
            } else if line[i..].starts_with("two") {
                Some(2)
            } else if line[i..].starts_with("three") {
                Some(3)
            } else if line[i..].starts_with("four") {
                Some(4)
            } else if line[i..].starts_with("five") {
                Some(5)
            } else if line[i..].starts_with("six") {
                Some(6)
            } else if line[i..].starts_with("seven") {
                Some(7)
            } else if line[i..].starts_with("eight") {
                Some(8)
            } else if line[i..].starts_with("nine") {
                Some(9)
            } else {
                None
            };
            if let Some(n) = number {
                if first.is_none() {
                    first = Some(n);
                    last = Some(n);
                } else {
                    last = Some(n);
                }
            }
        }
        res.push(first.unwrap() * 10 + last.unwrap());
    }
    res
}

fn main() {
    let input = read_input_1("data/1_trebuchet.txt");
    let sum = input.iter().sum::<u32>();
    println!("sum 1: {}", sum);
    let input = read_input_2("data/1_trebuchet.txt");
    let sum = input.iter().sum::<u32>();
    println!("sum 2: {}", sum);
}
