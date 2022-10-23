use crate::aoc_io::Solver;
use crate::parse;

use self::bt::{backtrack, State};

#[derive(Debug)]
pub struct Day15 {}

#[allow(clippy::new_without_default)]
impl Day15 {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

const MAX_INGREDIENTS: usize = 5;

/// Based on the book:
/// "Programming Challenges: The Programming Contest Training Manual"
/// by Steven Skiena and Miguel Revilla, Springer-Verlag, New York 2003.
/// http://www.amazon.com/exec/obidos/ASIN/0387001638/thealgorithmrepo/
mod bt {
    use super::{Ingredient, MAX_INGREDIENTS};
    use arrayvec::ArrayVec;
    use log::{debug, trace};

    pub struct State {
        pub best: u64,
        pub best_bounded: u64,
        pub facts: ArrayVec<Ingredient, MAX_INGREDIENTS>,
    }

    impl State {
        pub fn new() -> Self {
            Self {
                best: u64::MIN,
                best_bounded: u64::MIN,
                facts: ArrayVec::new(),
            }
        }
    }

    type Item = u32;

    const MAX_CANDIDATES: usize = 128;

    /// Given the (sub)solution a, what are the possible candidates for the next position?
    /// The candidates are stored in `c` and its length is returned.
    fn construct_candidates<const CAP: usize>(
        a: &[Item],
        c: &mut ArrayVec<Item, CAP>,
        _state: &State,
    ) {
        let teaspoons: Item = a.iter().sum();
        for i in 0..=100 - teaspoons {
            unsafe {
                c.push_unchecked(i);
            }
        }
    }

    #[inline]
    /// Check if a is a solution
    fn is_solution(a: &[Item], _state: &mut State) -> bool {
        let teaspoons: Item = a.iter().sum();
        teaspoons == 100
    }

    fn process_solution(a: &[Item], state: &mut State) {
        let capacity: i64 = 0.max(a.iter().enumerate().fold(0, |acc, (i, &amount)| {
            acc + (amount as i64) * (state.facts[i].capacity as i64)
        }));
        let durability: i64 = 0.max(a.iter().enumerate().fold(0, |acc, (i, &amount)| {
            acc + (amount as i64) * (state.facts[i].durability as i64)
        }));
        let flavor: i64 = 0.max(a.iter().enumerate().fold(0, |acc, (i, &amount)| {
            acc + (amount as i64) * (state.facts[i].flavor as i64)
        }));
        let texture: i64 = 0.max(a.iter().enumerate().fold(0, |acc, (i, &amount)| {
            acc + (amount as i64) * (state.facts[i].texture as i64)
        }));
        let cost = (capacity * durability * flavor * texture) as u64;
        debug!("found solution {:?} with cost {}", a, cost);
        if cost > state.best {
            state.best = cost;
        }

        let calories: i64 = a.iter().enumerate().fold(0, |acc, (i, &amount)| {
            acc + (amount as i64) * (state.facts[i].calories as i64)
        });
        if calories == 500 && cost > state.best_bounded {
            state.best_bounded = cost;
        }
    }

    pub fn backtrack(a: &mut [Item], state: &mut State) {
        go(a, 0, state)
    }

    // Generic backtracking algorithm. This can be re-used without modification.
    fn go(a: &mut [Item], k: usize, state: &mut State) {
        if k > a.len() {
            return;
        }
        {
            let candidate = unsafe { a.get_unchecked(..k) };
            if is_solution(candidate, state) {
                process_solution(candidate, state);
                return;
            }
        }
        let mut c = ArrayVec::<Item, MAX_CANDIDATES>::new();
        construct_candidates(unsafe { a.get_unchecked(..k) }, &mut c, state);
        trace!("k={}: found {} candidates", k, c.len());
        for &c_i in c.iter().take(c.len()) {
            unsafe {
                *a.get_unchecked_mut(k) = c_i;
            }
            // XXX: terminate early if possible
            go(a, k + 1, state);
        }
    }
}

impl Solver for Day15 {
    fn day(&self) -> i32 {
        15
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let mut state = State::new();

        let mut buf = input;
        while !buf.is_empty() {
            let (rest, _name) = parse::token(buf).unwrap();
            let (rest, capacity) = parse::integer(rest, true).unwrap();
            let (rest, durability) = parse::integer(rest, true).unwrap();
            let (rest, flavor) = parse::integer(rest, true).unwrap();
            let (rest, texture) = parse::integer(rest, true).unwrap();
            let (rest, calories) = parse::integer(rest, true).unwrap();

            let ingredient = Ingredient {
                capacity: capacity.try_into().unwrap(),
                durability: durability.try_into().unwrap(),
                flavor: flavor.try_into().unwrap(),
                texture: texture.try_into().unwrap(),
                calories: calories.try_into().unwrap(),
            };
            state.facts.push(ingredient);

            buf = parse::seek_next_line(rest);
        }

        let mut a: [u32; MAX_INGREDIENTS] = [0; MAX_INGREDIENTS];
        let n = state.facts.len();
        backtrack(&mut a[0..n], &mut state);

        let part1 = state.best;
        let part2 = state.best_bounded;

        (part1.to_string(), part2.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_io;

    #[test]
    fn examples() {
        let s = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
";
        let answer = Day15::new().solve(s.as_bytes());
        assert_eq!("62842880", answer.0);
        assert_eq!("57600000", answer.1);
    }

    #[test]
    fn part1_and_part2() {
        let mut d = Day15::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("21367368", answer.0);
        assert_eq!("1766400", answer.1);
    }
}
