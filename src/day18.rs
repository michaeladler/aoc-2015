use crate::aoc_io::Solver;
use log::debug;

#[derive(Debug)]
pub struct Day18 {}

const ON: u8 = b'#';
const OFF: u8 = b'.';

#[derive(Debug)]
struct Board {
    lights: [[bool; 100]; 100],
}

impl Board {
    pub const fn new() -> Self {
        Board {
            lights: [[false; 100]; 100],
        }
    }

    pub fn is_on(&self, x: i32, y: i32) -> bool {
        let n = self.lights.len() as i32;
        if (x < 0 || x >= n) || (y < 0 || y >= n) {
            return false;
        }
        let tmp = unsafe { self.lights.get_unchecked(x as usize) };
        let is_on = unsafe { *tmp.get_unchecked(y as usize) };
        is_on
    }

    pub unsafe fn set_light(&mut self, x: i32, y: i32, value: bool) {
        let tmp = self.lights.get_unchecked_mut(x as usize);
        *tmp.get_unchecked_mut(y as usize) = value;
    }

    pub fn count_neighbors_on(&self, x: i32, y: i32) -> usize {
        let mut count = 0;
        let deltas = [-1, 0, 1];
        for dx in deltas {
            for dy in deltas {
                if dx == 0 && dy == 0 {
                    continue;
                }
                count += self.is_on(x + dx, y + dy) as usize;
            }
        }
        count
    }

    pub fn count_on(&self) -> usize {
        let mut count = 0;
        let n = self.lights.len();
        unsafe {
            for x in 0..n {
                for y in 0..n {
                    let tmp = self.lights.get_unchecked(x);
                    count += *tmp.get_unchecked(y) as usize;
                }
            }
        }
        count
    }

    pub fn draw(&self, from: (i32, i32), to: (i32, i32)) {
        for y in from.1..=to.1 {
            for x in from.0..=to.0 {
                print!(
                    "{}",
                    if self.is_on(x, y) {
                        ON as char
                    } else {
                        OFF as char
                    }
                )
            }
            println!();
        }
    }
}

fn solveit(input: &[u8], max_x: i32, max_y: i32, steps: usize) -> (String, String) {
    let mut first_boards: [Board; 2] = [Board::new(), Board::new()];
    let mut second_boards: [Board; 2] = [Board::new(), Board::new()];
    {
        let mut y: i32 = 0;
        let mut x: i32 = 0;
        let board = &mut first_boards[0];
        let board2 = &mut second_boards[0];
        for &b in input.iter() {
            if b == b'\n' {
                x = 0;
                y += 1;
                continue;
            }
            unsafe {
                board.set_light(x, y, b == ON);
                board2.set_light(x, y, b == ON);
            }
            x += 1;
        }
    }

    let mut active_idx: usize = 0;
    for i in 0..steps {
        let new_active_idx = 1 - active_idx;

        for x in 0..max_x {
            for y in 0..max_y {
                let is_on = {
                    let current_board = unsafe { first_boards.get_unchecked(active_idx) };
                    let neighbor_count = current_board.count_neighbors_on(x, y);
                    match current_board.is_on(x, y) {
                        // A light which is *on* stays on when `2` or `3` neighbors are on
                        true => neighbor_count == 2 || neighbor_count == 3,
                        // A light which is *off* turns on if exactly `3` neighbors are on
                        false => neighbor_count == 3,
                    }
                };
                let new_board = unsafe { first_boards.get_unchecked_mut(new_active_idx) };
                unsafe {
                    new_board.set_light(x, y, is_on);
                }
            }
        }

        for x in 0..max_x {
            for y in 0..max_y {
                if (x == max_x - 1 || x == 0) && (y == max_y - 1 || y == 0) {
                    let new_board = unsafe { second_boards.get_unchecked_mut(new_active_idx) };
                    unsafe {
                        new_board.set_light(x, y, true);
                    }
                    continue;
                }

                let is_on = {
                    let current_board = unsafe { second_boards.get_unchecked(active_idx) };
                    let neighbor_count = current_board.count_neighbors_on(x, y);
                    match current_board.is_on(x, y) {
                        // A light which is *on* stays on when `2` or `3` neighbors are on
                        true => neighbor_count == 2 || neighbor_count == 3,
                        // A light which is *off* turns on if exactly `3` neighbors are on
                        false => neighbor_count == 3,
                    }
                };
                let new_board = unsafe { second_boards.get_unchecked_mut(new_active_idx) };
                unsafe {
                    new_board.set_light(x, y, is_on);
                }
            }
        }

        active_idx = new_active_idx;

        debug!("=== Step {} ===", i + 1);
        if log::log_enabled!(log::Level::Debug) {
            second_boards[active_idx].draw((0, 0), (5, 5));
        }
    }

    let part1 = first_boards[active_idx].count_on();
    let part2 = second_boards[active_idx].count_on();

    (part1.to_string(), part2.to_string())
}

#[allow(clippy::new_without_default)]
impl Day18 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solver for Day18 {
    fn day(&self) -> i32 {
        18
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        solveit(input, 100, 100, 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_io;

    #[test]
    fn part1_example() {
        let s = b".#.#.#
...##.
#....#
..#...
#.#..#
####..";
        let answer = solveit(s, 6, 6, 4);
        assert_eq!("4", answer.0);
    }

    #[test]
    fn part2_example() {
        let s = b"##.#.#
...##.
#....#
..#...
#.#..#
####.#";
        let answer = solveit(s, 6, 6, 5);
        assert_eq!("17", answer.1);
    }

    #[test]
    fn part1_and_part2() {
        let mut d = Day18::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("768", answer.0);
        assert_eq!("781", answer.1);
    }
}
