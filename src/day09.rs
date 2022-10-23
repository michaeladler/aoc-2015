use ahash::AHashMap;
use compact_str::CompactString;
use log::{debug, trace};

use crate::aoc_io::Solver;
use crate::parse;

/// Based on the book:
/// "Programming Challenges: The Programming Contest Training Manual"
/// by Steven Skiena and Miguel Revilla, Springer-Verlag, New York 2003.
/// http://www.amazon.com/exec/obidos/ASIN/0387001638/thealgorithmrepo/
mod bt {
    use super::MAX_CITIES;
    use arrayvec::ArrayVec;
    use log::trace;

    #[derive(Debug)]
    pub struct State {
        pub best: usize,
        pub worst: usize,
        pub distances: [[u64; MAX_CITIES]; MAX_CITIES],
        pub num_cities: usize,
    }

    impl State {
        pub fn new(num_cities: usize) -> Self {
            Self {
                best: usize::MAX,
                worst: usize::MIN,
                distances: [[0; MAX_CITIES]; MAX_CITIES],
                num_cities,
            }
        }
    }

    type Item = usize;

    const MAX_CANDIDATES: usize = MAX_CITIES;

    /// Given the (sub)solution a, what are the possible candidates for the next position?
    /// The candidates are stored in `c` and its length is returned.
    fn construct_candidates<const CAP: usize>(
        a: &[Item],
        c: &mut ArrayVec<Item, CAP>,
        state: &State,
    ) {
        // bitset
        let blacklist: u64 = a.iter().fold(0, |acc, x| acc | 1 << x);
        for i in 0..state.num_cities {
            if blacklist & 1 << i == 0 {
                unsafe {
                    c.push_unchecked(i);
                }
            }
        }
    }

    #[inline]
    /// Check if a is a solution
    fn is_solution(a: &[Item], state: &mut State) -> bool {
        a.len() == state.num_cities
    }

    fn process_solution(a: &[Item], state: &mut State) {
        let cost: usize = unsafe {
            let mut cost = 0;
            let mut from_id = *a.get_unchecked(0);
            for &to_id in a.iter().skip(1) {
                cost += *state.distances.get_unchecked(from_id).get_unchecked(to_id) as usize;
                from_id = to_id;
            }
            cost
        };

        if cost < state.best {
            state.best = cost;
        }
        if cost > state.worst {
            state.worst = cost;
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

const MAX_CITIES: usize = 8;

#[derive(Debug)]
pub struct Day09 {}

impl Day09 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Day09 {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct CityDistance {
    from: CompactString,
    to: CompactString,
    dist: u64,
}

fn parse_line(buf: &[u8]) -> parse::ParseResult<&[u8], CityDistance> {
    let (buf, from) = parse::token(buf)?;
    let (buf, to) = parse::token(&buf[4..])?;
    let (buf, dist) = parse::positive(&buf[3..], false)?;
    trace!("{} to {} = {}", from, to, dist);
    Some((buf, CityDistance { from, to, dist }))
}

impl Solver for Day09 {
    fn day(&self) -> i32 {
        9
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        // Represent a city as usize
        let mut city_to_id: AHashMap<CompactString, usize> = AHashMap::with_capacity(MAX_CITIES);
        let mut state = bt::State::new(0);

        let mut buf = input;
        while !buf.is_empty() {
            let x = parse_line(buf).unwrap();
            let citydist = x.1;
            buf = parse::seek_next_line(x.0);

            let maybe_from = city_to_id.get(&citydist.from);
            let from_id = match maybe_from {
                Some(&id) => id,
                None => {
                    let old_id = state.num_cities;
                    city_to_id.insert(citydist.from, state.num_cities);
                    state.num_cities += 1;
                    old_id
                }
            };

            let maybe_to = city_to_id.get(&citydist.to);
            let to_id = match maybe_to {
                Some(&id) => id,
                None => {
                    let old_id = state.num_cities;
                    city_to_id.insert(citydist.to, state.num_cities);
                    state.num_cities += 1;
                    old_id
                }
            };

            trace!("{} to {} = {}", from_id, to_id, citydist.dist);
            state.distances[from_id][to_id] = citydist.dist;
            state.distances[to_id][from_id] = citydist.dist;
        }

        debug!("{} cities", state.num_cities);
        debug!("{:?}", state.distances);

        let mut a: [usize; MAX_CITIES] = [0; MAX_CITIES];
        bt::backtrack(&mut a[0..state.num_cities], &mut state);

        let part1 = state.best;
        let part2 = state.worst;
        (part1.to_string(), part2.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_io;

    #[test]
    fn part1_example() {
        let bufs = vec![(
            "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
",
            605,
        )];

        for (s, answer) in bufs {
            let mut d = Day09::new();
            assert_eq!(answer.to_string(), d.solve(s.as_bytes()).0);
        }
    }

    #[test]
    fn part2_example() {
        let bufs = vec![(
            "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
",
            982,
        )];

        for (s, answer) in bufs {
            let mut d = Day09::new();
            assert_eq!(answer.to_string(), d.solve(s.as_bytes()).1);
        }
    }

    #[test]
    fn part1_and_part2() {
        let mut d = Day09::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("141", answer.0);
        assert_eq!("736", answer.1);
    }
}
