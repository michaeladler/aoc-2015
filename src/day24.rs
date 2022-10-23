use std::cmp::Ordering;
use std::collections::VecDeque;
use std::thread;

use ahash::AHashSet;
use arrayvec::ArrayVec;

use crate::aoc_io::Solver;
use crate::parse;

#[derive(Debug)]
pub struct Day24 {}

const MAX_ITEMS: usize = 32;

#[allow(clippy::new_without_default)]
impl Day24 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
struct Group {
    /// the indexes of the chosen weights, starting at 0
    indexes: u64,
    total_weight: i32,
}

impl Group {
    pub fn quantum_entanglement(&self, weights: &[i32]) -> i64 {
        weights
            .iter()
            .enumerate()
            .filter_map(|(i, weight)| {
                if self.indexes & (1 << i) != 0 {
                    Some(*weight as i64)
                } else {
                    None
                }
            })
            .product()
    }
}

impl Solver for Day24 {
    fn day(&self) -> i32 {
        24
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let mut weights: ArrayVec<i32, MAX_ITEMS> = ArrayVec::new();
        let mut buf = input;
        while !buf.is_empty() {
            let (rest, x) = parse::integer(buf, false).unwrap();
            unsafe {
                weights.push_unchecked(x as i32);
            }
            buf = parse::seek_next_line(rest);
        }
        let weights = weights;
        let sum: i32 = weights.iter().sum();

        let weights2 = weights.clone();

        let thread1 = thread::spawn(move || {
            let solutions = find_groups(&weights, sum / 3);
            let mut min_entanglement = i64::MAX;
            for grp in solutions {
                let entanglement = grp.quantum_entanglement(&weights);
                if entanglement < min_entanglement {
                    min_entanglement = entanglement;
                }
            }
            min_entanglement
        });

        let part2;
        {
            let solutions = find_groups(&weights2, sum / 4);
            let mut min_entanglement = i64::MAX;
            for grp in solutions {
                let entanglement = grp.quantum_entanglement(&weights2);
                if entanglement < min_entanglement {
                    min_entanglement = entanglement;
                }
            }
            part2 = min_entanglement;
        };

        let part1 = thread1.join().unwrap();

        (part1.to_string(), part2.to_string())
    }
}

fn find_groups(weights: &[i32], target_weight: i32) -> ArrayVec<Group, MAX_ITEMS> {
    let mut solutions: ArrayVec<Group, MAX_ITEMS> = ArrayVec::new();

    // bfs
    let start = Group::default();
    let mut queue: VecDeque<Group> = VecDeque::with_capacity(1024);
    let mut visited: AHashSet<Group> = AHashSet::with_capacity(1024);
    queue.push_back(start.clone());
    visited.insert(start);
    while let Some(current) = queue.pop_front() {
        for (i, weight) in weights.iter().enumerate() {
            if current.indexes & (1 << i) == 0 {
                // weight with index i is not yet used
                let mut candidate = current.clone();
                candidate.indexes |= 1 << i;
                candidate.total_weight = current.total_weight + weight;

                match candidate.total_weight.cmp(&target_weight) {
                    Ordering::Less => {
                        // need to add another weight
                        if !visited.contains(&candidate) {
                            queue.push_back(candidate.clone());
                            visited.insert(candidate);
                        }
                    }
                    Ordering::Equal => unsafe {
                        solutions.push_unchecked(candidate);
                    },
                    _ => {}
                }
            }
        }
        if !solutions.is_empty() {
            break;
        }
    }
    solutions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_io;

    #[test]
    fn part1_and_part2() {
        let mut d = Day24::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("10439961859", answer.0);
        assert_eq!("72050269", answer.1);
    }
}
