use crate::aoc_io::Solver;

#[derive(Debug)]
pub struct Day11 {}

#[allow(clippy::new_without_default)]
impl Day11 {
    pub fn new() -> Self {
        Self {}
    }
}

const PW_LEN: usize = 8;

#[inline]
fn is_valid(password: &[u8]) -> bool {
    let mut has_increasing_straight_three = false;
    let mut has_bad_char = false;
    let mut pairs: u64 = 0;

    let n = password.len();
    for i in 0..n - 2 {
        // Passwords may not contain the letters `i`, `o`, or `l`
        let b = unsafe { *password.get_unchecked(i) };
        if b == b'i' || b == b'o' || b == b'l' {
            has_bad_char = true;
        }

        // Passwords must contain at least two different, non-overlapping pairs of letters, like `aa`, `bb``
        let c = unsafe { *password.get_unchecked(i + 1) };
        if b == c {
            let idx = (b - b'a') as usize;
            pairs |= 1 << idx;
        }

        // include one increasing straight of at least three letters, like `abc`, `bcd`, `cde`,
        let d = unsafe { *password.get_unchecked(i + 2) };
        if b + 1 == c && c + 1 == d {
            has_increasing_straight_three = true;
        }
    }

    // two last letters
    for i in n - 2..n {
        // Passwords may not contain the letters `i`, `o`, or `l`
        let b = unsafe { *password.get_unchecked(i) };
        if b == b'i' || b == b'o' || b == b'l' {
            has_bad_char = true;
        }

        // Passwords must contain at least two different, non-overlapping pairs of letters, like `aa`, `bb``
        let c = unsafe { *password.get_unchecked(i + 1) };
        if b == c {
            let idx = (b - b'a') as usize;
            pairs |= 1 << idx;
        }
    }

    // last letter
    let i = n - 1;
    // Passwords may not contain the letters `i`, `o`, or `l`
    let b = unsafe { *password.get_unchecked(i) };
    if b == b'i' || b == b'o' || b == b'l' {
        has_bad_char = true;
    }

    let has_two_non_overlapping = pairs.count_ones() > 1;

    has_increasing_straight_three && !has_bad_char && has_two_non_overlapping
}

#[inline]
fn next_char(b: u8) -> (u8, bool) {
    match b {
        b'z' => (b'a', true), // overflow
        _ => (b + 1, false),
    }
}

fn next_password(password: &mut [u8]) {
    loop {
        let mut pos: usize = PW_LEN - 1;
        // generate next pw
        loop {
            let is_overflow = unsafe {
                let (next_b, is_overflow) = next_char(*password.get_unchecked(pos));
                *password.get_unchecked_mut(pos) = next_b;
                is_overflow
            };
            if !is_overflow {
                break;
            }
            pos -= 1;
        }
        if is_valid(password) {
            return;
        }
    }
}

impl Solver for Day11 {
    fn day(&self) -> i32 {
        11
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let mut password: [u8; PW_LEN] = [0; PW_LEN];
        for (i, &b) in input.iter().take(PW_LEN).enumerate() {
            password[i] = b;
        }

        next_password(&mut password);
        let part1 = unsafe { std::str::from_utf8_unchecked(&password) }.to_string();
        next_password(&mut password);
        let part2 = unsafe { std::str::from_utf8_unchecked(&password) }.to_string();

        (part1, part2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_io;

    #[test]
    fn test_is_valid() {
        let bufs = vec![
            ("hijklmmn", false),
            ("abbceffg", false),
            ("abbcegjk", false),
            ("hijklmmn", false),
        ];

        for (s, answer) in bufs {
            assert_eq!(answer, is_valid(s.as_bytes()));
        }
    }

    #[test]
    fn part1_and_part2() {
        let mut d = Day11::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("cqjxxyzz", answer.0);
        assert_eq!("cqkaabcc", answer.1);
    }
}
