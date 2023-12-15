use std::collections::HashMap;
use std::fs;

fn parse_input(path: &str) -> Vec<String> {
    let file = fs::read_to_string(path).expect("cannot read file");
    let line = file.lines().next().unwrap();
    line.split(',').map(|s| s.to_string()).collect()
}

struct Box {
    lens: HashMap<String, usize>,
    order: Vec<String>,
}

impl Box {
    fn new() -> Box {
        Box {
            lens: HashMap::new(),
            order: Vec::new(),
        }
    }

    fn add(&mut self, label: &str, focal_length: usize) {
        if !self.lens.contains_key(label) {
            self.order.push(label.to_string());
        }
        self.lens.insert(label.to_string(), focal_length);
    }

    fn remove(&mut self, label: &str) {
        if self.lens.contains_key(label) {
            self.lens.remove(label);
            self.order.retain(|s| s != label);
        }
    }
}

fn hash(input: &str) -> usize {
    let mut res = 0;
    for c in input.chars() {
        res = ((res + c as usize) * 17) % 256;
    }
    res
}

fn exercise_1(input: &[String]) -> usize {
    input.iter().map(|s| hash(s)).sum()
}

fn exercise_2(input: &[String]) -> usize {
    let mut boxes = HashMap::new();
    for i in 0..256 {
        boxes.insert(i, Box::new());
    }

    for line in input {
        // println!("After \"{}\":", line);
        if line.contains('-') {
            let mut parts = line.split('-');
            let label = parts.next().unwrap();
            let target_box = boxes.get_mut(&hash(label)).unwrap();
            target_box.remove(label);
        } else if line.contains('=') {
            let mut parts = line.split('=');
            let label = parts.next().unwrap();
            let focal_length = parts.next().unwrap().parse::<usize>().unwrap();
            let target_box = boxes.get_mut(&hash(label)).unwrap();
            target_box.add(label, focal_length);
        } else {
            panic!("invalid input");
        }
        // for i in 0..256 {
        //     let target_box = boxes.get_mut(&i).unwrap();
        //     if target_box.lens.is_empty() {
        //         continue;
        //     }
        //     print!("Box {}: ", i);
        //     for label in &target_box.order {
        //         print!("[{} {}] ", label, target_box.lens[label]);
        //     }
        //     println!();
        // }
        // println!();
    }
    let mut res = 0;
    for i in 0..256 {
        let target_box = boxes.get_mut(&i).unwrap();
        for (j, label) in target_box.order.iter().enumerate() {
            let length = target_box.lens[label];
            res += (i + 1) * (j + 1) * length
        }
    }
    res
}

fn main() {
    let input = parse_input("data/15_input.txt");

    println!("Exercise 1: {}", exercise_1(&input));
    println!("Exercise 2: {}", exercise_2(&input));
}
