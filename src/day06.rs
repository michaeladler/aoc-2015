use crate::aoc_io::Solver;
use crate::parse;

#[derive(Debug)]
pub struct Day06 {}

impl Day06 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Day06 {
    fn default() -> Self {
        Self::new()
    }
}

const GRID_SIZE: usize = 1000;

struct Grid {
    grid: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new() -> Self {
        let mut grid = Vec::with_capacity(GRID_SIZE);
        for _ in 0..GRID_SIZE {
            grid.push(vec![false; GRID_SIZE]);
        }
        Self { grid }
    }

    pub fn process(&mut self, instruction: Instruction, rectangle: &Rectangle) {
        let x1 = rectangle.first_corner.0 as usize;
        let y1 = rectangle.first_corner.1 as usize;
        let x2 = rectangle.second_corner.0 as usize;
        let y2 = rectangle.second_corner.1 as usize;
        debug_assert!(x1 <= x2);
        debug_assert!(y1 <= y2);
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.grid[x][y] = match instruction {
                    Instruction::On => true,
                    Instruction::Off => false,
                    Instruction::Toggle => !(self.grid[x][y]),
                };
            }
        }
    }

    pub fn lit_count(&self) -> u32 {
        self.grid
            .iter()
            .map(|row| row.iter().map(|&b| b as u32).sum::<u32>())
            .sum()
    }
}

struct BrightnessGrid {
    grid: Vec<Vec<i32>>,
}

impl BrightnessGrid {
    pub fn new() -> Self {
        let mut grid = Vec::with_capacity(GRID_SIZE);
        for _ in 0..GRID_SIZE {
            grid.push(vec![0; GRID_SIZE]);
        }
        Self { grid }
    }

    pub fn process(&mut self, instruction: Instruction, rectangle: &Rectangle) {
        let x1 = rectangle.first_corner.0 as usize;
        let y1 = rectangle.first_corner.1 as usize;
        let x2 = rectangle.second_corner.0 as usize;
        let y2 = rectangle.second_corner.1 as usize;
        debug_assert!(x1 <= x2);
        debug_assert!(y1 <= y2);
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.grid[x][y] = match instruction {
                    Instruction::On => self.grid[x][y] + 1,
                    Instruction::Off => std::cmp::max(0, self.grid[x][y] - 1),
                    Instruction::Toggle => self.grid[x][y] + 2,
                };
            }
        }
    }

    pub fn brightness_count(&self) -> u32 {
        self.grid
            .iter()
            .map(|row| row.iter().map(|&b| b as u32).sum::<u32>())
            .sum()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Rectangle {
    first_corner: (u32, u32),
    second_corner: (u32, u32),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Instruction {
    On,
    Off,
    Toggle,
}

fn parse_rectangle(line: &[u8]) -> Option<Rectangle> {
    let (line, x1) = parse::positive(line, true)?;
    let (line, y1) = parse::positive(line, true)?;
    let (line, x2) = parse::positive(line, true)?;
    let (_, y2) = parse::positive(line, true)?;
    Some(Rectangle {
        first_corner: (x1 as u32, y1 as u32),
        second_corner: (x2 as u32, y2 as u32),
    })
}

fn parse_instruction(line: &[u8]) -> Option<(Instruction, Rectangle)> {
    // turn off, turn on, toggle
    let rectangle = parse_rectangle(line).expect("parse_rectangle");
    if line[1] == b'o' {
        return Some((Instruction::Toggle, rectangle));
    }
    if line[6] == b'n' {
        return Some((Instruction::On, rectangle));
    }
    if line[6] == b'f' {
        return Some((Instruction::Off, rectangle));
    }
    None
}

impl Solver for Day06 {
    fn day(&self) -> i32 {
        6
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let mut grid = Grid::new();
        let mut brightness_grid = BrightnessGrid::new();

        let mut line_start: usize = 0;
        for (i, &b) in input.iter().enumerate() {
            if b == b'\n' {
                let line = &input[line_start..i];
                let (instruction, rectangle) = parse_instruction(line).expect("instruction");
                grid.process(instruction, &rectangle);
                brightness_grid.process(instruction, &rectangle);
                line_start = i + 1;
            }
        }

        let part1 = grid.lit_count();
        let part2 = brightness_grid.brightness_count();

        (part1.to_string(), part2.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_io;

    #[test]
    fn part1_and_part2() {
        let mut d = Day06::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("569999", answer.0);
        assert_eq!("17836115", answer.1);
    }
}
