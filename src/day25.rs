use log::debug;

use crate::aoc_io::Solver;
use crate::math;
use crate::parse;

#[derive(Debug)]
pub struct Day25 {}

#[allow(clippy::new_without_default)]
impl Day25 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solver for Day25 {
    fn day(&self) -> i32 {
        25
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let (rest, row) = parse::positive(input, true).unwrap();
        let (_, col) = parse::positive(rest, true).unwrap();
        debug!("row: {row}, col: {col}");

        // Objective: Find i such that (row, col) is the i-th element in the sequence (i is 1-based)
        // Facts:
        // - element (1,n) is the 1+2+...+n = n*(n+1)/2 element in the sequence
        // - if (1,n) is the i-th element, then:
        //      - (2,n) is the (i+n)-th element
        //      - (3,n) is the (i+n+(n+1))=(i+2n+1)-th element
        //      - (4,n) is the (i+n+(n+1)+(n+2))=(i+3n+3) element
        //      - (j,n) is the (i+(j-1)*n+ ((j-2)*(j-1)/2)) element

        let top = (col * (col + 1)) / 2;
        let i = top + (row - 1) * col + (((row - 2) * (row - 1)) / 2);
        debug!("(1,{col}) is the {top}-th element in the sequence");
        debug!("({row},{col}) is the {i}-th element in the sequence");

        let seed: u64 = 20151125;
        let base: u64 = 252533;
        let exp = i - 1;
        let modulus = 33554393;

        let part1 = (seed * math::pow_mod(base, exp, modulus)) % modulus;

        (part1.to_string(), "".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_io;

    #[test]
    fn part1() {
        let mut d = Day25::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("9132360", answer.0);
    }
}
