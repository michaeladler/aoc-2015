use ahash::AHashMap;
use arrayvec::ArrayVec;
use compact_str::CompactString;
use log::debug;

use crate::aoc_io::Solver;
use crate::parse;

const MAX_PEOPLE: usize = 10;

#[derive(Debug)]
pub struct Day13 {
    max_happiness: i32,
    costs: [[i32; MAX_PEOPLE]; MAX_PEOPLE],
}

#[allow(clippy::new_without_default)]
impl Day13 {
    pub fn new() -> Self {
        Self {
            max_happiness: i32::MIN,
            costs: [[0; MAX_PEOPLE]; MAX_PEOPLE],
        }
    }
}

impl Day13 {
    #[inline]
    fn calc_cost(&self, i: usize, j: usize) -> i32 {
        unsafe {
            self.costs.get_unchecked(i).get_unchecked(j)
                + self.costs.get_unchecked(j).get_unchecked(i)
        }
    }

    /// Heap's algorithm generates all possible permutations of n objects.
    /// https://en.wikipedia.org/wiki/Heap%27s_algorithm
    fn permutations(&mut self, a: &mut [usize], k: usize) {
        if k == 1 {
            let n = a.len();
            let mut happiness: i32 = self.calc_cost(0, unsafe { *a.get_unchecked(0) })
                + self.calc_cost(0, unsafe { *a.get_unchecked(n - 1) });
            for (i, &person_id) in a.iter().take(n - 1).enumerate() {
                happiness += self.calc_cost(person_id, unsafe { *a.get_unchecked(i + 1) });
            }

            debug!("{:?}: {}", a, happiness);
            if happiness > self.max_happiness {
                self.max_happiness = happiness;
            }
            return;
        }

        for i in 0..k {
            self.permutations(a, k - 1);

            let idx = (k % 2 == 0) as usize * i;
            unsafe {
                let tmp = *a.get_unchecked(idx);
                *a.get_unchecked_mut(idx) = *a.get_unchecked(k - 1);
                *a.get_unchecked_mut(k - 1) = tmp;
            }
        }
    }
}

impl Solver for Day13 {
    fn day(&self) -> i32 {
        13
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let mut name_to_id: AHashMap<CompactString, usize> = AHashMap::with_capacity(8);
        {
            let mut idx: usize = 0;
            let mut buf = input;
            while !buf.is_empty() {
                let (rest, who) = parse::token(buf).unwrap();
                //  skip " would "
                let rest = &rest[7..];
                let rest = parse::skip_ws(rest);
                let (rest, action) = parse::token(rest).unwrap();
                let rest = parse::skip_ws(rest);
                let (rest, happiness) = parse::positive(rest, false).unwrap();
                // skip " happiness units by sitting next to "
                let rest = &rest[36..];
                let (rest, other) = parse::token(rest).unwrap();
                let amount = if action.starts_with('g') {
                    happiness as i32
                } else {
                    -(happiness as i32)
                };
                debug!("({}, {}): {}", who, other, amount);

                let who_id = *name_to_id.entry(who).or_insert(idx);
                if who_id == idx {
                    idx += 1;
                }

                let other_id = *name_to_id.entry(other).or_insert(idx);
                if other_id == idx {
                    idx += 1;
                }

                self.costs[who_id][other_id] = amount;

                buf = parse::seek_next_line(rest);
            }
        }
        let name_to_id = name_to_id;

        debug!("{:?}", name_to_id);
        debug!("{:?}", self.costs);

        let mut a: ArrayVec<usize, MAX_PEOPLE> = ArrayVec::new();
        for i in 1..=MAX_PEOPLE {
            a.push(i);
        }

        // position 0 is fixed (person with id 0), and we only permutate the rest, i.e. (n-1)!
        // permutations
        let n = name_to_id.len() - 1;
        self.permutations(&mut a[0..n], n);
        let part1 = self.max_happiness;

        // part 2
        let n = n + 1;
        a[n - 1] = n;
        for entry in self.costs[n].iter_mut() {
            // your happiness wouldn't really go up or down regardless of who you sit next to
            *entry = 0;
        }
        for entry in self.costs.iter_mut() {
            entry[n] = 0;
        }

        self.max_happiness = i32::MIN;
        self.permutations(&mut a[0..n], n);
        let part2 = self.max_happiness;
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
            "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
",
            330,
        )];

        for (s, answer) in bufs {
            let mut d = Day13::new();
            assert_eq!(answer.to_string(), d.solve(s.as_bytes()).0);
        }
    }

    #[test]
    fn part1_and_part2() {
        let mut d = Day13::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("733", answer.0);
        assert_eq!("725", answer.1);
    }
}
