use serde_json::{self, Value};

use crate::aoc_io::Solver;

#[derive(Debug)]
pub struct Day12 {}

#[allow(clippy::new_without_default)]
impl Day12 {
    pub fn new() -> Self {
        Self {}
    }
}
fn eval(value: &Value) -> (i64, i64) {
    match value {
        Value::Number(number) => {
            let val = number.as_i64().unwrap();
            return (val, val);
        }
        Value::Array(arr) => {
            let mut sum1 = 0;
            let mut sum2 = 0;
            for x in arr {
                let res = eval(x);
                sum1 += res.0;
                sum2 += res.1;
            }
            return (sum1, sum2);
        }
        Value::Object(obj) => {
            let mut sum1 = 0;
            let mut sum2 = 0;
            let mut found_red = false;
            for (key, val) in obj {
                if key == "red" || val == "red" {
                    found_red = true;
                }
                let res = eval(val);
                sum1 += res.0;
                sum2 += res.1;
            }
            return (sum1, if found_red { 0 } else { sum2 });
        }
        _ => {}
    };
    (0, 0)
}

impl Solver for Day12 {
    fn day(&self) -> i32 {
        12
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let value: Value = serde_json::from_slice(input).unwrap();
        let (part1, part2) = eval(&value);
        (part1.to_string(), part2.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_io;

    #[test]
    fn part1_example() {
        let bufs = vec![
            ("[1,2,3]", 6),
            ("[[[3]]]", 3),
            ("{\"a\":{\"b\":4},\"c\":-1}", 3),
            ("{\"a\":[-1,1]}", 0),
            ("[]", 0),
            ("{}", 0),
        ];

        for (s, answer) in bufs {
            let mut d = Day12::new();
            assert_eq!(answer.to_string(), d.solve(s.as_bytes()).0);
        }
    }

    #[test]
    fn part1_and_part2() {
        let mut d = Day12::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("191164", answer.0);
        assert_eq!("87842", answer.1);
    }
}
