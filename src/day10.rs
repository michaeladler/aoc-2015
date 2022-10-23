use crate::aoc_io::Solver;

#[derive(Debug)]
pub struct Day10 {}

impl Day10 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Day10 {
    fn default() -> Self {
        Self::new()
    }
}

const ITERATIONS: usize = 50;

#[inline]
fn add_to_seq(b: u8, counter: u64, out: &mut Vec<u8>) {
    // get digits and store them
    let mut digits: [u8; 32] = [0; 32];
    let mut idx_digits = 0;
    let mut counter = counter;
    while counter != 0 {
        unsafe {
            *digits.get_unchecked_mut(idx_digits) = (counter % 10) as u8;
        }
        idx_digits += 1;
        counter /= 10;
    }
    let digits = &digits[0..idx_digits];
    for &d in digits.iter().rev() {
        out.push(b'0' + d);
    }
    out.push(b);
}

impl Solver for Day10 {
    fn day(&self) -> i32 {
        10
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let mut seq1 = Vec::with_capacity(6_000_000);
        let mut seq2 = Vec::with_capacity(6_000_000);

        for &b in input.iter() {
            if b == b'\n' {
                break;
            }
            seq1.push(b);
        }

        let mut part1: usize = 0;
        let mut part2: usize = 0;

        let mut old_seq = &seq1;
        let mut new_seq = &mut seq2;
        let mut idx = 0;

        for iteration in 1..=ITERATIONS {
            let new_idx = 1 - idx;
            new_seq.clear();

            let mut counter: u64 = 1;
            let mut prev: u8 = unsafe { *old_seq.get_unchecked(0) };
            for &b in &old_seq[1..] {
                if b == prev {
                    counter += 1;
                    continue;
                }
                add_to_seq(prev, counter, new_seq);
                prev = b;
                counter = 1;
            }
            add_to_seq(prev, counter, new_seq);

            if iteration == 40 {
                part1 = new_seq.len();
            } else if iteration == 50 {
                part2 = new_seq.len();
            }

            if idx == 0 {
                new_seq = &mut seq1;
                old_seq = &seq2;
            } else {
                new_seq = &mut seq2;
                old_seq = &seq1;
            }

            idx = new_idx;
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
        let mut d = Day10::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("329356", answer.0);
        assert_eq!("4666278", answer.1);
    }
}
