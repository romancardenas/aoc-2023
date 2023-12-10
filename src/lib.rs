/// Greatest common divisor
pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

/// Least common multiple
pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
