use crate::aoc_io::Solver;
use ahash::AHashSet;

#[derive(Debug)]
pub struct Day03 {}

impl Day03 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Day03 {
    fn default() -> Self {
        Self::new()
    }
}

impl Solver for Day03 {
    fn day(&self) -> i32 {
        3
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;

        let mut map_part1: AHashSet<(i32, i32)> = AHashSet::new();
        let mut map_part2: AHashSet<(i32, i32)> = AHashSet::new();

        let mut coords_part2: [[i32; 2]; 2] = [[0, 0], [0, 0]];
        let mut index: usize = 0;

        map_part1.insert((0, 0));
        map_part2.insert((0, 0));
        for c in input.iter() {
            match c {
                b'^' => {
                    y -= 1;
                    coords_part2[index][1] -= 1;
                }
                b'v' => {
                    y += 1;
                    coords_part2[index][1] += 1;
                }
                b'>' => {
                    x += 1;
                    coords_part2[index][0] += 1;
                }
                b'<' => {
                    x -= 1;
                    coords_part2[index][0] -= 1;
                }
                _ => {}
            }
            map_part1.insert((x, y));
            map_part2.insert((coords_part2[index][0], coords_part2[index][1]));
            index = 1 - index;
        }

        let part1 = map_part1.len();
        let part2 = map_part2.len();

        (part1.to_string(), part2.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_io;

    use super::*;

    #[test]
    fn part1_example() {
        let bufs = vec![(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)];

        for (s, answer) in bufs {
            let mut d = Day03::new();
            assert_eq!(answer.to_string(), d.solve(s.as_bytes()).0);
        }
    }

    #[test]
    fn part2_example() {
        let bufs = vec![("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)];

        for (s, answer) in bufs {
            let mut d = Day03::new();
            assert_eq!(answer.to_string(), d.solve(s.as_bytes()).1);
        }
    }

    #[test]
    fn part1_and_part2() {
        let mut d = Day03::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("2565", answer.0);
        assert_eq!("2639", answer.1);
    }
}
