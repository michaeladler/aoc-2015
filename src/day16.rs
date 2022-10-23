use crate::{aoc_io::Solver, parse};
use arrayvec::ArrayVec;
use log::debug;

#[derive(Debug)]
pub struct Day16 {}

#[allow(clippy::new_without_default)]
impl Day16 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Default)]
struct Sue {
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

const MAX_SUES: usize = 500;

impl Solver for Day16 {
    fn day(&self) -> i32 {
        16
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let sues = parse_input(input);

        let mut matches: [u32; MAX_SUES] = [0; MAX_SUES];
        let mut matches_loose: [u32; MAX_SUES] = [0; MAX_SUES];
        for (i, sue) in sues.iter().enumerate() {
            let mut count_normal = 0;
            let mut count_loose = 0;
            if sue.children == Some(3) {
                count_normal += 1;
                count_loose += 1;
            }
            if sue.samoyeds == Some(2) {
                count_normal += 1;
                count_loose += 1;
            }
            if sue.akitas == Some(0) {
                count_normal += 1;
                count_loose += 1;
            }
            if sue.vizslas == Some(0) {
                count_normal += 1;
                count_loose += 1;
            }
            if sue.cars == Some(2) {
                count_normal += 1;
                count_loose += 1;
            }
            if sue.perfumes == Some(1) {
                count_normal += 1;
                count_loose += 1;
            }

            if let Some(amount) = sue.cats {
                count_normal += (amount == 7) as u32;
                count_loose += (amount > 7) as u32;
            }
            if let Some(amount) = sue.trees {
                count_normal += (amount == 3) as u32;
                count_loose += (amount > 3) as u32;
            }

            if let Some(amount) = sue.pomeranians {
                count_normal += (amount == 3) as u32;
                count_loose += (amount <= 3) as u32;
            }
            if let Some(amount) = sue.goldfish {
                count_normal += (amount == 5) as u32;
                count_loose += (amount <= 5) as u32;
            }

            unsafe {
                *matches.get_unchecked_mut(i) = count_normal;
                *matches_loose.get_unchecked_mut(i) = count_loose;
            }
        }

        let part1 = matches
            .iter()
            .enumerate()
            .max_by(|x, y| x.1.cmp(y.1))
            .map(|x| x.0 + 1)
            .unwrap();
        let part2 = matches_loose
            .iter()
            .enumerate()
            .max_by(|x, y| x.1.cmp(y.1))
            .map(|x| x.0 + 1)
            .unwrap();

        (part1.to_string(), part2.to_string())
    }
}

fn parse_input(input: &[u8]) -> ArrayVec<Sue, 500> {
    let mut sues: ArrayVec<Sue, MAX_SUES> = ArrayVec::new();
    let mut buf: &[u8] = input;
    while !buf.is_empty() {
        buf = parse::skip_ws(parse::seek(buf, b':'));
        let mut sue = Sue::default();
        loop {
            let (rest, token) = parse::token(buf).unwrap();
            let (rest, amount) = parse::positive(rest, true).unwrap();
            let amount = amount as u32;
            match token.as_str() {
                "children" => {
                    sue.children = Some(amount);
                }
                "cats" => {
                    sue.cats = Some(amount);
                }
                "samoyeds" => {
                    sue.samoyeds = Some(amount);
                }
                "pomeranians" => {
                    sue.pomeranians = Some(amount);
                }
                "akitas" => {
                    sue.akitas = Some(amount);
                }
                "vizslas" => {
                    sue.vizslas = Some(amount);
                }
                "goldfish" => {
                    sue.goldfish = Some(amount);
                }
                "trees" => {
                    sue.trees = Some(amount);
                }
                "cars" => {
                    sue.cars = Some(amount);
                }
                "perfumes" => {
                    sue.perfumes = Some(amount);
                }
                _ => {
                    panic!("Invalid token")
                }
            }
            if let Some(b'\n') = rest.first() {
                break;
            }
            debug_assert_eq!(rest[0], b',');
            buf = &rest[2..];
        }
        debug!("{:?}", sue);
        sues.push(sue);
        buf = parse::seek_next_line(buf);
    }
    sues
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_io;

    #[test]
    fn part1_and_part2() {
        let mut d = Day16::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("40", answer.0);
        assert_eq!("241", answer.1);
    }
}
