use arrayvec::ArrayVec;
use log::debug;

use crate::aoc_io::Solver;
use crate::parse;

#[derive(Debug)]
pub struct Day17 {}

#[allow(clippy::new_without_default)]
impl Day17 {
    pub fn new() -> Self {
        Self {}
    }
}

const MAX_CONTAINERS: usize = 20;

fn parse_input(input: &[u8]) -> ArrayVec<u32, MAX_CONTAINERS> {
    let mut result = ArrayVec::new();
    let mut buf = input;
    while !buf.is_empty() {
        let (rest, val) = parse::integer(buf, false).unwrap();
        result.push(val as u32);
        buf = parse::seek_next_line(rest);
    }
    result
}

fn solve(containers: &[u32], total_expected: u32) -> (u32, u32) {
    // powerset from https://www.geeksforgeeks.org/power-set/
    let n = containers.len();
    let pow_set_size: usize = 1 << n;
    debug!("set_size: {}, pow_set_size: {}", n, pow_set_size);

    let mut part1 = 0;
    // array index: the number of containers used to obtain `total_expected` volume
    // array value: how often this was found
    let mut containers_used: [u32; 20] = [0; 20];

    // Run from counter 000..0 to 111..1
    for counter in 0..pow_set_size {
        // Check if jth bit in the counter is set
        let mut sum = 0;
        for j in 0..n {
            // If set then print jth element from set
            if (counter & (1 << j)) != 0 {
                sum += unsafe { containers.get_unchecked(j) };
            }
        }
        if sum == total_expected {
            part1 += 1;
            containers_used[counter.count_ones() as usize] += 1;
        }
    }

    debug!("containers_used: {:?}", containers_used);
    let part2 = containers_used.iter().find(|x| **x != 0).unwrap();

    (part1, *part2)
}

impl Solver for Day17 {
    fn day(&self) -> i32 {
        17
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let sizes = parse_input(input);
        debug!("sizes: {:?}", sizes);
        let (part1, part2) = solve(&sizes, 150);
        (part1.to_string(), part2.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_io;

    #[test]
    fn example() {
        let s = b"20
15
10
5
5
";

        let sizes = parse_input(s);
        let answer = solve(&sizes, 25);
        assert_eq!(4, answer.0);
        assert_eq!(3, answer.1);
    }

    #[test]
    fn part1_and_part2() {
        let mut d = Day17::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("1304", answer.0);
        assert_eq!("18", answer.1);
    }
}
