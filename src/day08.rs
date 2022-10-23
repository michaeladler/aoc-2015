use crate::aoc_io::Solver;
use log::debug;

#[derive(Debug)]
pub struct Day08 {}

impl Day08 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Day08 {
    fn default() -> Self {
        Self::new()
    }
}

fn is_hex(b: u8) -> bool {
    (b'0'..=b'9').contains(&b) || (b'a'..=b'f').contains(&b)
}

impl Solver for Day08 {
    fn day(&self) -> i32 {
        8
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let mut part1: i64 = 0;
        let mut part2: i64 = 0;

        let mut code_len: i64 = 0;
        let mut string_len: i64 = 0;
        let mut encoded_len: i64 = 2;
        let mut i: usize = 0;
        while i < input.len() {
            let &b = unsafe { input.get_unchecked(i) };
            if b == b'\n' {
                string_len -= 2; // strip surrounding quotes ""
                debug!(
                    "code_len: {}, string_len: {}, encoded_len: {}",
                    code_len, string_len, encoded_len
                );
                part1 += code_len - string_len;
                part2 += encoded_len - code_len; // two surrounding quotes

                code_len = 0;
                string_len = 0;
                encoded_len = 2;
                i += 1;
                continue;
            }

            let next = unsafe { input.get_unchecked(i + 1) };
            match (b, next) {
                (b'\\', b'\\') | (b'\\', b'"') => {
                    debug!("escaped at pos {}", i + 1);
                    code_len += 2;
                    string_len += 1;
                    encoded_len += 4;
                    i += 2;
                    continue;
                }
                (b'\\', b'x') => {
                    // check if the following two bytes are a hex number
                    if let (Some(&x), Some(&y)) = (input.get(i + 2), input.get(i + 3)) {
                        if is_hex(x) && is_hex(y) {
                            debug!("found escaped hex at pos {}", i,);
                            code_len += 4;
                            string_len += 1;
                            encoded_len += 5;
                            i += 4;
                            continue;
                        }
                    }
                }
                _ => {}
            }

            let char_encoded_len = match b {
                b'\\' | b'"' => 2,
                _ => 1,
            };
            encoded_len += char_encoded_len;

            code_len += 1;
            string_len += 1;
            i += 1;
        }
        (part1.to_string(), part2.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_io;

    #[test]
    fn part1_and_part2() {
        let mut d = Day08::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("1342", answer.0);
        assert_eq!("2074", answer.1);
    }
}
