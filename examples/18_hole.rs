use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    dir: char,
    steps: usize,
    color: usize,
}

impl Instruction {
    fn adapt(&mut self) {
        self.steps = self.color >> 4;
        self.dir = match self.color & 0xF {
            0x0 => 'R',
            0x1 => 'D',
            0x2 => 'L',
            0x3 => 'U',
            _ => panic!("invalid encoding"),
        };
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} (#{:06x})", self.dir, self.steps, self.color)
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(' ').collect::<Vec<_>>();

        let dir = s[0].chars().next().unwrap();
        let steps = s[1].parse().unwrap();
        let color = s[2].trim_start_matches("(#").trim_end_matches(')');
        let color = usize::from_str_radix(color, 16).unwrap();
        Ok(Self { dir, steps, color })
    }
}

fn parse_input(path: &str) -> Vec<Instruction> {
    let file = fs::read_to_string(path).expect("cannot read file");
    file.lines().map(|line| line.parse().unwrap()).collect()
}

fn compute_path(instructions: &[Instruction]) -> Vec<(i64, i64)> {
    let mut pos = (0, 0);
    let mut res = vec![pos];
    for Instruction {
        dir,
        steps,
        color: _,
    } in instructions
    {
        pos = match dir {
            'U' => (pos.0 - *steps as i64, pos.1),
            'D' => (pos.0 + *steps as i64, pos.1),
            'L' => (pos.0, pos.1 - *steps as i64),
            'R' => (pos.0, pos.1 + *steps as i64),
            _ => panic!("unknown direction"),
        };
        res.push(pos);
    }
    res
}

fn calc_filled(instructions: &[Instruction]) -> i64 {
    let path = compute_path(instructions);
    let mut area: i64 = 0;
    let mut perimeter: i64 = 0;
    for i in 0..(path.len() - 1) {
        let (from, to) = (path[i], path[i + 1]);
        area += from.0 * to.1 - from.1 * to.0;
        if from.0 == to.0 {
            perimeter += i64::abs(from.1 - to.1);
        } else if from.1 == to.1 {
            perimeter += i64::abs(from.0 - to.0);
        }
    }
    // shoelace formula: https://en.wikipedia.org/wiki/Shoelace_formula
    let area = i64::abs(area) / 2;
    // shoelace formula does not count the half of the perimeter (it is outside the polygon)
    let area = area + perimeter / 2;
    // it does not count one edge of the polygon (the one between the last and the first point)
    area + 1
}

fn main() {
    let mut instructions = parse_input("data/18_input.txt");
    // exercise_1(&instructions);

    let area = calc_filled(&instructions);
    println!("area: {}", area);

    instructions.iter_mut().for_each(|i| i.adapt());
    let area = calc_filled(&instructions);
    println!("area: {}", area);
}
