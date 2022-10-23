use ahash::AHashMap;

use crate::aoc_io::Solver;

#[derive(Debug)]
pub struct Day05 {}

impl Day05 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Day05 {
    fn default() -> Self {
        Self::new()
    }
}

fn is_nice(word: &[u8]) -> (bool, bool) {
    let mut vowel_count: i32 = 0;
    let mut twice_count: i32 = 0;
    let mut forbidden_count: i32 = 0;

    let mut prev = b'\0';
    let mut prev_prev = b'\0';

    // ('x', 'y') -> (starting positions of second char)
    let mut pairs: AHashMap<(char, char), Vec<usize>> = AHashMap::new();
    let mut bracket_count: i32 = 0;

    for (i, &b) in word.iter().enumerate() {
        vowel_count += (b == b'a' || b == b'e' || b == b'i' || b == b'o' || b == b'u') as i32;
        twice_count += (prev == b) as i32;
        forbidden_count += ((prev == b'a' && b == b'b')
            || (prev == b'c' && b == b'd')
            || (prev == b'p' && b == b'q')
            || (prev == b'x' && b == b'y')) as i32;

        if let Some(positions) = pairs.get_mut(&(prev as char, b as char)) {
            positions.push(i);
        } else {
            let mut v = Vec::with_capacity(8);
            v.push(i);
            pairs.insert((prev as char, b as char), v);
        }

        bracket_count += (prev_prev == b) as i32;

        prev_prev = prev;
        prev = b;
    }

    // check pairs for overlaps; they overlap if the starting positions are
    // consecutive, e.g. 2 and 3
    let mut has_pair_twice_non_overlapping = false;
    'outer: for starting_positions in pairs.values() {
        // we are good as soon as there is a gap
        let prev = starting_positions[0];
        for pos in starting_positions.iter().skip(1) {
            let delta = pos - prev;
            if delta > 1 {
                has_pair_twice_non_overlapping = true;
                break 'outer;
            }
        }
    }

    let nice_part1 = vowel_count >= 3 && twice_count > 0 && forbidden_count == 0;
    let nice_part2 = bracket_count > 0 && has_pair_twice_non_overlapping;
    (nice_part1, nice_part2)
}

impl Solver for Day05 {
    fn day(&self) -> i32 {
        5
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let chunk_size = 17;
        let mut part1: i32 = 0;
        let mut part2: i32 = 0;
        // chunks include the trailing newline, and the last line must end
        // with a newline as well
        for line in input.chunks(chunk_size) {
            let stripped = &line[0..line.len() - 1];
            debug_assert_eq!(chunk_size - 1, stripped.len());
            let (p1, p2) = is_nice(stripped);
            part1 += p1 as i32;
            part2 += p2 as i32;
        }
        (part1.to_string(), part2.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_io;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_is_nice() {
        let bufs = vec![
            ("ugknbfddgicrmopn", true),
            ("aaa", true),
            ("jchzalrnumimnmhp", false),
            ("haegwjzuvuyypxyu", false),
            ("dvszwmarrgswjxmb", false),
        ];
        for (s, answer) in bufs {
            assert_eq!(answer, is_nice(s.as_bytes()).0);
        }
    }

    #[test]
    fn test_is_nice_2() {
        init();
        let bufs = vec![
            ("aaa", false),
            ("qjhvhtzxzqqjkmpb", true),
            ("xxyxx", true),
            ("uurcxstgmygtbstg", false),
            ("ieodomkazucvgmuy", false),
        ];
        for (s, answer) in bufs {
            assert_eq!(answer, is_nice(s.as_bytes()).1);
        }
    }

    #[test]
    fn part1_and_part2() {
        let mut d = Day05::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("236", answer.0);
        assert_eq!("51", answer.1);
    }
}
