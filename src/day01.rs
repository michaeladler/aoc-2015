use crate::aoc_io::Solver;

#[derive(Debug)]
pub struct Day01 {}

impl Default for Day01 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day01 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solver for Day01 {
    fn day(&self) -> i32 {
        1
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let mut part1: i64 = 0;
        let mut part2 = None;
        for (pos, &x) in (1..=input.len()).zip(input.iter()) {
            part1 += if x as char == '(' { 1 } else { -1 };
            if part2.is_none() && part1 < 0 {
                part2 = Some(pos);
            }
        }

        (part1.to_string(), part2.unwrap_or_default().to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_io;

    use super::*;

    #[test]
    fn part1_example() {
        let bufs = vec![
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ];

        for (s, answer) in bufs {
            let mut day01 = Day01::new();
            assert_eq!(answer.to_string(), day01.solve(s.as_bytes()).0);
        }
    }

    #[test]
    fn part2_example() {
        let bufs = vec![(")", 1), ("()())", 5)];

        for (s, answer) in bufs {
            let mut day01 = Day01::new();
            assert_eq!(answer.to_string(), day01.solve(s.as_bytes()).1);
        }
    }

    #[test]
    fn part1_and_part2() {
        let mut d = Day01::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("74", answer.0);
        assert_eq!("1795", answer.1);
    }
}
