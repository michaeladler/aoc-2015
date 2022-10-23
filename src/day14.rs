use arrayvec::ArrayVec;

use log::debug;

use crate::aoc_io::Solver;
use crate::parse;

#[derive(Debug)]
pub struct Day14 {}

#[allow(clippy::new_without_default)]
impl Day14 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
struct Reindeer {
    speed: u32,
    duration: u32,
    pause: u32,
    state: State,
}

#[derive(Debug, Clone, Copy)]
enum State {
    Flying(u32),
    Resting(u32),
}

const MAX_DEERS: usize = 10;

impl Solver for Day14 {
    fn day(&self) -> i32 {
        14
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let mut buf = input;
        let mut animals: ArrayVec<Reindeer, MAX_DEERS> = ArrayVec::new();
        while !buf.is_empty() {
            let (rest, _who) = parse::token(buf).unwrap();
            let (rest, speed) = parse::positive(rest, true).unwrap();
            let (rest, duration) = parse::positive(rest, true).unwrap();
            let (rest, pause) = parse::positive(rest, true).unwrap();
            animals.push(Reindeer {
                speed: speed as u32,
                duration: duration as u32,
                pause: pause as u32,
                state: State::Flying(duration as u32),
            });
            buf = parse::seek_next_line(rest);
        }

        let mut distances: [u64; MAX_DEERS] = [0; MAX_DEERS];
        let distances = &mut distances[0..animals.len()];

        let mut extra_points: [u64; MAX_DEERS] = [0; MAX_DEERS];
        let extra_points = &mut extra_points[0..animals.len()];

        debug!("animals: {:?}", animals);
        let n = 2503;
        for tick in 1..=n {
            for (i, deer) in animals.iter_mut().enumerate() {
                let state = deer.state;
                match state {
                    State::Flying(delta) => {
                        unsafe {
                            *distances.get_unchecked_mut(i) += deer.speed as u64;
                        }
                        if delta == 1 {
                            deer.state = State::Resting(deer.pause);
                        } else {
                            deer.state = State::Flying(delta - 1);
                        }
                    }
                    State::Resting(delta) => {
                        if delta == 1 {
                            deer.state = State::Flying(deer.duration);
                        } else {
                            deer.state = State::Resting(delta - 1);
                        }
                    }
                }
            }

            let max_val = *distances
                .iter()
                .enumerate()
                .max_by(|x, y| x.1.cmp(y.1))
                .unwrap()
                .1;
            for (i, &val) in distances.iter().enumerate() {
                if val == max_val {
                    unsafe {
                        *extra_points.get_unchecked_mut(i) += 1;
                    }
                }
            }

            debug!(
                "{}: distances={:?}, extra_points={:?}",
                tick, distances, extra_points
            );
        }

        let part1 = distances.iter().max().unwrap_or(&0);
        let part2 = extra_points.iter().max().unwrap_or(&0);

        (part1.to_string(), part2.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_io;

    #[test]
    fn part1_and_part2() {
        let mut d = Day14::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("2696", answer.0);
        assert_eq!("1084", answer.1);
    }
}
