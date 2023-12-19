use core::panic;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn apply_rule<'a>(&self, rule: &'a str) -> Option<&'a str> {
        let rule = rule.split(':').collect::<Vec<_>>();
        match rule.len() {
            1 => Some(rule[0]),
            2 => {
                let mut condition = rule[0].trim().chars();
                let attr = condition.next().unwrap();
                let cond = condition.next().unwrap();
                let val: usize = condition.collect::<String>().parse().unwrap();
                let num = match attr {
                    'x' => self.x,
                    'm' => self.m,
                    'a' => self.a,
                    's' => self.s,
                    _ => panic!("invalid attribute"),
                };
                match cond {
                    '<' => {
                        if num < val {
                            Some(rule[1])
                        } else {
                            None
                        }
                    }
                    '>' => {
                        if num > val {
                            Some(rule[1])
                        } else {
                            None
                        }
                    }
                    _ => panic!("invalid condition"),
                }
            }
            _ => panic!("invalid rule"),
        }
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .map(|s| s.split('=').last().unwrap())
            .collect::<Vec<_>>();
        let x = s[0].parse().unwrap();
        let m = s[1].parse().unwrap();
        let a = s[2].parse().unwrap();
        let s = s[3].parse().unwrap();
        Ok(Self { x, m, a, s })
    }
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{x={}, m={}, a={}, s={}}}",
            self.x, self.m, self.a, self.s
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    name: String,
    rules: Vec<String>,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.name, self.rules)
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_end_matches('}').split('{').collect::<Vec<_>>();
        let name = s[0].to_string();
        let rules = s[1].split(',').map(|s| s.to_string()).collect::<Vec<_>>();
        Ok(Self { name, rules })
    }
}

fn parse_input(path: &str) -> (HashMap<String, Instruction>, Vec<Part>) {
    let (mut instructions, mut parts) = (HashMap::new(), vec![]);
    let file = fs::read_to_string(path).expect("cannot read file");
    let mut lines = file.lines().into_iter();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let instruction: Instruction = line.parse().unwrap();
        instructions.insert(instruction.name.clone(), instruction);
    }

    while let Some(line) = lines.next() {
        let part = line.parse().unwrap();
        parts.push(part);
    }

    (instructions, parts)
}

fn exercise_1(instructions: &HashMap<String, Instruction>, parts: &[Part]) -> usize {
    let (mut accepted, mut rejected) = (vec![], vec![]);

    for part in parts {
        print!("- {}: in", part);
        let mut done = false;
        let mut instruction = instructions.get("in").unwrap();
        loop {
            for rule in &instruction.rules {
                if let Some(next) = part.apply_rule(rule) {
                    print!(" -> {}", next);
                    match next {
                        "A" => {
                            done = true;
                            accepted.push(part);
                        }
                        "R" => {
                            done = true;
                            rejected.push(part);
                        }
                        _ => {
                            if let Some(next_instruction) = instructions.get(next) {
                                instruction = next_instruction;
                            }
                        }
                    }
                    break;
                }
            }
            if done {
                break;
            }
        }
        println!();
    }
    accepted.iter().map(|p| p.x + p.m + p.a + p.s).sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PartRange {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl PartRange {
    fn new() -> Self {
        Self {
            x: (1, 4_000),
            m: (1, 4_000),
            a: (1, 4_000),
            s: (1, 4_000),
        }
    }

    fn len(&self) -> usize {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }

    fn get_range(&self, attr: char) -> (usize, usize) {
        match attr {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("invalid attribute"),
        }
    }

    fn set_range(&mut self, attr: char, range: (usize, usize)) {
        match attr {
            'x' => self.x = range,
            'm' => self.m = range,
            'a' => self.a = range,
            's' => self.s = range,
            _ => panic!("invalid attribute"),
        }
    }

    fn apply_rule(self, rule: &str) -> (&str, Option<Self>, Option<Self>) {
        let rule = rule.split(':').collect::<Vec<_>>();
        match rule.len() {
            1 => (rule[0], Some(self), None),
            2 => {
                let mut condition = rule[0].trim().chars();
                let attr = condition.next().unwrap();
                let cond = condition.next().unwrap();
                let val: usize = condition.collect::<String>().parse().unwrap();
                let range = self.get_range(attr);

                let next = rule[1];
                let (mut accepted, mut rejected) = (None, None);

                match cond {
                    '<' => {
                        if range.1 < val {
                            accepted = Some(self);
                        } else if range.0 > val {
                            rejected = Some(self);
                        } else {
                            let (mut acc, mut rej) = (self, self);
                            acc.set_range(attr, (range.0, val - 1));
                            rej.set_range(attr, (val, range.1));
                            (accepted, rejected) = (Some(acc), Some(rej));
                        }
                    }
                    '>' => {
                        if range.0 > val {
                            accepted = Some(self);
                        } else if range.1 < val {
                            rejected = Some(self);
                        } else {
                            let (mut acc, mut rej) = (self, self);
                            acc.set_range(attr, (val + 1, range.1));
                            rej.set_range(attr, (range.0, val));
                            (accepted, rejected) = (Some(acc), Some(rej));
                        }
                    }
                    _ => panic!("invalid condition"),
                };
                (next, accepted, rejected)
            }
            _ => panic!("invalid rule"),
        }
    }
}

fn exercise_2(instructions: &HashMap<String, Instruction>) -> usize {
    let (mut accepted, mut rejected) = (vec![], vec![]);

    let mut new_parts = vec![(PartRange::new(), "in")];

    while let Some((mut part, instruction)) = new_parts.pop() {
        let instruction = instructions.get(instruction).unwrap();
        for rule in &instruction.rules {
            let (next, acc, rej) = part.apply_rule(rule);
            if let Some(a) = acc {
                match next {
                    "A" => {
                        accepted.push(a);
                    }
                    "R" => {
                        rejected.push(a);
                    }
                    _ => {
                        new_parts.push((a, next));
                    }
                }
            }
            if let Some(pending) = rej {
                part = pending;
            } else {
                break;
            }
        }
    }
    accepted.iter().map(|p| p.len()).sum()
}

fn main() {
    let (instructions, parts) = parse_input("data/19_input.txt");

    println!("exercise 1: {}", exercise_1(&instructions, &parts));
    println!("exercise 2: {}", exercise_2(&instructions));
    // exercise_1(&instructions);
}
