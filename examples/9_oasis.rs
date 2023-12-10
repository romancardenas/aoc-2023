use std::collections::vec_deque::VecDeque;

fn parse_input(path: &str) -> Vec<Vec<isize>> {
    let mut res = Vec::new();
    let file = std::fs::read_to_string(path).expect("cannot read file");
    for line in file.lines() {
        let samples = line
            .split(' ')
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        res.push(samples);
    }
    res
}

fn exercise1(samples: &[Vec<isize>]) -> isize {
    let mut res = 0;
    for sample in samples {
        let mut history = vec![sample.clone()];
        // derive history
        while history.last().unwrap().iter().any(|n| *n != 0) {
            let prev = history.last().unwrap();
            let mut next = Vec::new();
            for i in 0..prev.len() - 1 {
                next.push(prev[i + 1] - prev[i]);
            }
            history.push(next);
        }
        // integrate history
        let mut delta = 0;
        for h in history.iter_mut().rev() {
            let last = h.last().unwrap();
            let next = last + delta;
            h.push(next);
            delta = next;
        }
        res += history.first().unwrap().last().unwrap();
    }
    res
}

fn exercise2(samples: &[Vec<isize>]) -> isize {
    let mut res = 0;
    for sample in samples {
        println!("sample: {:?}", sample);
        let mut history = vec![VecDeque::from(sample.clone())];
        // derive history
        while history.last().unwrap().iter().any(|n| *n != 0) {
            let prev = history.last().unwrap();
            let mut next = VecDeque::new();
            for i in 0..prev.len() - 1 {
                next.push_back(prev[i + 1] - prev[i]);
            }
            println!("    next: {:?}", next);
            history.push(next);
        }
        // integrate history
        let mut delta = 0;
        for h in history.iter_mut().rev() {
            let first = h.front().unwrap();
            let zero = first - delta;
            h.push_front(zero);
            delta = zero;
            println!("    h: {:?}", h);
        }
        res += history.first().unwrap().front().unwrap();
    }
    res
}

fn main() {
    let scenario = parse_input("data/9_input.txt");
    println!("exercise1: {}", exercise1(&scenario));
    println!("exercise2: {}", exercise2(&scenario));
}
