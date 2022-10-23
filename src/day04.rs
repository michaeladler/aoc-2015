use crate::aoc_io::Solver;
use crate::convert::byte_to_hex;

#[derive(Debug)]
pub struct Day04 {
    hex_out: [char; 6],
    pub part1: Option<String>,
    pub part2: Option<String>,
}

impl Default for Day04 {
    fn default() -> Self {
        Self::new()
    }
}

impl Day04 {
    pub fn new() -> Self {
        Self {
            hex_out: ['0'; 6],
            part1: None,
            part2: None,
        }
    }

    pub fn check_candidate(&mut self, input: &[u8], seed_len: usize) {
        let digest = md5::compute(input);
        prefix_hex(digest, &mut self.hex_out);
        if self.hex_out[0..5] == ['0'; 5] {
            if self.part1.is_none() {
                self.part1 = Some(input.iter().skip(seed_len).map(|&x| x as char).collect());
            }
            if self.hex_out[5] == '0' && self.part2.is_none() {
                self.part2 = Some(input.iter().skip(seed_len).map(|&x| x as char).collect());
            }
        }
    }
}

fn prefix_hex(digest: md5::Digest, out: &mut [char; 6]) {
    for (i, &b) in digest.iter().take(3).enumerate() {
        let (x1, x2) = byte_to_hex(b);
        out[2 * i] = x1;
        out[2 * i + 1] = x2;
    }
}

impl Solver for Day04 {
    fn day(&self) -> i32 {
        4
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let mut candidate: [u8; 32] = [0; 32];
        let mut last: usize = 0;
        for (i, &b) in input.iter().enumerate() {
            if b == b'\n' {
                break;
            }
            candidate[i] = b;
            last = i;
        }
        let seed_len = last + 1;

        let mut digits: [u8; 8] = [0; 8]; // more than 8 digits would be too slow anyway
        'outer: for _x in 1..=10_000_000 {
            let mut x = _x;
            // get digits
            let mut n: usize = digits.len() - 1;
            while x != 0 {
                let digit = (x % 10) as u8;
                x /= 10;
                digits[n] = b'0' + digit;
                n -= 1;
            }
            let last = seed_len + (7 - n);
            candidate[seed_len..last].copy_from_slice(&digits[n + 1..]);
            self.check_candidate(&candidate[0..last], seed_len);
            if self.part1.is_some() && self.part2.is_some() {
                break 'outer;
            }
        }

        (self.part1.clone().unwrap(), self.part2.clone().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_io;

    use super::*;

    #[test]
    fn check_candidate_part1() {
        let input: [char; 14] = [
            'b', 'g', 'v', 'y', 'z', 'd', 's', 'v', '2', '5', '4', '5', '7', '5',
        ];
        let input = input.map(|x| x as u8);

        let mut d = Day04::new();
        d.check_candidate(&input, 8);
        assert_eq!(Some(String::from("254575")), d.part1);
        assert_eq!(true, d.part2.is_none());
    }

    #[test]
    fn check_candidate_part2() {
        let input: [char; 15] = [
            'b', 'g', 'v', 'y', 'z', 'd', 's', 'v', '1', '0', '3', '8', '7', '3', '6',
        ];
        let input = input.map(|x| x as u8);

        let mut d = Day04::new();
        d.check_candidate(&input, 8);
        assert_eq!(Some(String::from("1038736")), d.part2);
    }

    #[test]
    fn part1_and_part2() {
        let mut d = Day04::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("254575", answer.0);
        assert_eq!("1038736", answer.1);
    }
}
