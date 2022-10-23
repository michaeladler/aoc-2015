use std::collections::BTreeMap;

use crate::aoc_io::Solver;
use crate::parse;

use ahash::{HashSet, HashSetExt};
use num_prime::nt_funcs::factorize64;

#[derive(Debug)]
pub struct Day20 {}

#[allow(clippy::new_without_default)]
impl Day20 {
    pub fn new() -> Self {
        Self {}
    }
}

// based on https://www.planetmath.org/FormulaForSumOfDivisors
fn sum_divisors(factorization: &BTreeMap<u64, usize>) -> u64 {
    factorization
        .iter()
        .fold(1, |acc, (&p, &m)| acc * (p.pow(m as u32 + 1) - 1) / (p - 1))
}

fn is_visited_part2(house: u64, elf: u64) -> bool {
    // each Elf will stop after delivering presents to 50 houses.
    house <= 50 * elf
}

// NOTE: this drains factorization
fn all_divisors(factorization: &mut Vec<(u64, usize)>) -> HashSet<u64> {
    let mut result = HashSet::new();
    if let Some((p, m)) = factorization.pop() {
        let other_result = all_divisors(factorization);
        result.reserve(other_result.len() * (m + 1) + m);
        for i in 0..=m {
            result.insert(p.pow(i as u32));
            for x in &other_result {
                result.insert(x * p.pow(i as u32));
            }
        }
    }
    result
}

impl Solver for Day20 {
    fn day(&self) -> i32 {
        20
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let input: u64 = parse::integer(input, false).unwrap().1 as u64;
        let lower = input / 10;

        let mut part1: Option<u64> = None;
        let mut part2: Option<u64> = None;
        for house in 100000..=input {
            let factorization = factorize64(house);
            if part1.is_none() && sum_divisors(&factorization) >= lower {
                part1 = Some(house);
            }

            let mut factorization = factorization.iter().map(|x| (*x.0, *x.1)).collect();
            let divisors = all_divisors(&mut factorization);
            let sum: u64 = divisors
                .iter()
                .filter(|&elf| is_visited_part2(house, *elf))
                .sum();
            if part2.is_none() && 11 * sum >= input {
                part2 = Some(house);
            }

            if part1.is_some() && part2.is_some() {
                break;
            }
        }

        (part1.unwrap().to_string(), part2.unwrap().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_io;

    #[test]
    fn part1_and_part2() {
        let mut d = Day20::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("665280", answer.0);
        assert_eq!("705600", answer.1);
    }
}
