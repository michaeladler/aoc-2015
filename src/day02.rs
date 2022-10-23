use std::cmp;

use crate::aoc_io::Solver;
use crate::parse;

#[derive(Debug)]
pub struct Day02 {}

impl Day02 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Day02 {
    fn default() -> Self {
        Self::new()
    }
}

impl Solver for Day02 {
    fn day(&self) -> i32 {
        2
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let mut part1 = 0;
        let mut part2 = 0;

        let mut input_buf = input;
        'outer: loop {
            match parse::positive(input_buf, false) {
                None => {
                    break 'outer;
                }
                Some((buf, l)) => {
                    let (buf, w) = parse::positive(buf, true).unwrap();
                    let (buf, h) = parse::positive(buf, true).unwrap();
                    let buf = &buf[std::cmp::min(1, buf.len())..]; // skip newline
                    input_buf = buf;

                    let (area1, perimeter1) = (l * w, 2 * (l + w));
                    let (area2, perimeter2) = (w * h, 2 * (w + h));
                    let (area3, perimeter3) = (h * l, 2 * (h + l));
                    let min_area = cmp::min(area1, cmp::min(area2, area3));
                    let min_perimeter = cmp::min(perimeter1, cmp::min(perimeter2, perimeter3));
                    part1 += 2 * (area1 + area2 + area3) + min_area;
                    part2 += min_perimeter + l * w * h;
                }
            }
        }

        (part1.to_string(), part2.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_io;

    use super::*;

    #[test]
    fn part1_example() {
        let bufs = vec![("2x3x4", 58), ("1x1x10", 43)];

        for (s, answer) in bufs {
            let mut d = Day02::new();
            assert_eq!(answer.to_string(), d.solve(s.as_bytes()).0);
        }
    }

    #[test]
    fn part2_example() {
        let bufs = vec![("2x3x4", 34), ("1x1x10", 14)];

        for (s, answer) in bufs {
            let mut d = Day02::new();
            assert_eq!(answer.to_string(), d.solve(s.as_bytes()).1);
        }
    }

    #[test]
    fn part1_and_part2() {
        let mut d = Day02::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("1586300", answer.0);
        assert_eq!("3737498", answer.1);
    }
}
