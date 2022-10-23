use crate::aoc_io::Solver;
use crate::parse;
use arrayvec::ArrayVec;
use compact_str::CompactString;
use log::{debug, trace};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

use ahash::AHashSet;

#[derive(Debug)]
pub struct Day19 {}

#[allow(clippy::new_without_default)]
impl Day19 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solver for Day19 {
    fn day(&self) -> i32 {
        19
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let (start, rules) = parse_day19(input);

        debug!("rules: {:?}", rules);
        debug!("start: {}", start);

        let part1 = calibrate(&rules, &start);
        let part2 = min_steps(&rules, &start);

        (part1.to_string(), part2.unwrap_or(-1).to_string())
    }
}

/// Number of molecules that can be generated in one step
fn calibrate(rules: &[Rule], start: &CompactString) -> usize {
    let mut mutations: AHashSet<CompactString> = AHashSet::with_capacity(rules.len());
    let n = start.len();
    for offset in 0..n {
        trace!("offset: {}", offset);
        for rule in rules {
            let end = offset + rule.lhs.len();
            if end <= n {
                let chunk = &start[offset..end];
                if chunk == rule.lhs {
                    trace!("rule {:?} matches at pos {}", rule, offset);
                    let modified = apply_rule(start, offset, rule);
                    trace!("modified: {}", modified);
                    mutations.insert(modified);
                }
            }
        }
    }
    mutations.len()
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Node {
    word: CompactString,
    depth: i32,
}

// The priority queue depends on `Ord`.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .word
            .len()
            .cmp(&self.word.len())
            .then_with(|| self.depth.cmp(&other.depth))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// What is the minimum number of steps to go from start to target?
fn min_steps(rules: &[Rule], target: &CompactString) -> Option<i32> {
    let mut rev_rules: ArrayVec<Rule, 64> = ArrayVec::new();
    for r in rules.iter() {
        unsafe {
            rev_rules.push_unchecked(r.reverse());
        }
    }
    debug!("rev_rules: {:?}", rev_rules);

    let mut heap = BinaryHeap::with_capacity(8192);
    let mut seen: AHashSet<CompactString> = AHashSet::with_capacity(8192);

    let start = CompactString::new("e");
    seen.insert(target.clone());
    heap.push(Node {
        word: target.clone(),
        depth: 0,
    });

    // dfs
    while let Some(current) = heap.pop() {
        debug!("current length: {}", current.word.len());
        if current.word == start {
            return Some(current.depth);
        }
        let n = current.word.len();
        if n > target.len() {
            continue;
        }
        debug!(
            "depth {}, len: {}: constructing for {}",
            current.depth,
            current.word.len(),
            current.word
        );
        let old_count = heap.len();
        for offset in 0..n {
            for rule in &rev_rules {
                let end = offset + rule.lhs.len();
                if end <= n && &current.word[offset..end] == rule.lhs {
                    let child = apply_rule(&current.word, offset, rule);
                    trace!(
                        "{:?} matches at pos {}. net gain: {}",
                        rule,
                        offset,
                        child.len() as i64 - current.word.len() as i64
                    );
                    if !seen.contains(&child) {
                        seen.insert(child.clone());
                        heap.push(Node {
                            word: child,
                            depth: current.depth + 1,
                        });
                    }
                }
            }
        }
        debug!(
            "added {} new entries. stack size: {}",
            heap.len() - old_count,
            heap.len()
        );
    }
    None
}

pub struct Rule {
    pub lhs: CompactString,
    pub rhs: CompactString,
}

impl Rule {
    pub fn reverse(&self) -> Rule {
        Rule {
            lhs: self.rhs.clone(),
            rhs: self.lhs.clone(),
        }
    }
}

impl std::fmt::Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{} => {}", self.lhs, self.rhs)
    }
}

fn apply_rule(src: &CompactString, pos: usize, rule: &Rule) -> CompactString {
    let lhs = &rule.lhs;
    let rhs = &rule.rhs;
    let n = src.len();
    let mut result: Vec<u8> = Vec::with_capacity(if n + rhs.len() > rhs.len() {
        n + rhs.len() - lhs.len()
    } else {
        1
    });
    unsafe {
        for i in 0..pos {
            result.push(*src.as_bytes().get_unchecked(i));
        }
    }
    for &x in rhs.as_bytes() {
        result.push(x);
    }
    unsafe {
        for i in pos + lhs.len()..n {
            result.push(*src.as_bytes().get_unchecked(i));
        }
    }
    unsafe { CompactString::new(String::from_utf8_unchecked(result)) }
}

fn parse_day19(input: &[u8]) -> (CompactString, ArrayVec<Rule, 64>) {
    let mut buf = input;
    let mut rules: ArrayVec<Rule, 64> = ArrayVec::new();
    let mut start = None;
    while !buf.is_empty() {
        if let Some((rest, lhs)) = parse::token(buf) {
            if rest.len() > 5 {
                if let Some((_, rhs)) = parse::token(&rest[4..]) {
                    trace!("{} => {}", lhs, rhs);
                    unsafe {
                        rules.push_unchecked(Rule { lhs, rhs });
                    }
                } else {
                    unsafe {
                        start = Some(CompactString::new(std::str::from_utf8_unchecked(
                            lhs.as_bytes(),
                        )));
                    }
                }
            } else {
                unsafe {
                    start = Some(CompactString::new(std::str::from_utf8_unchecked(
                        lhs.as_bytes(),
                    )));
                }
            }

            buf = rest;
        }
        buf = parse::seek_next_line(buf);
    }
    (start.expect("found no start"), rules)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_io;

    #[test]
    fn test_apply_rule() {
        let s = CompactString::new("HOH");
        let rule = Rule {
            lhs: CompactString::new("H"),
            rhs: CompactString::new("HO"),
        };
        let result = apply_rule(&s, 0, &rule);
        assert_eq!(CompactString::new("HOOH"), result);
        let reverse_result = apply_rule(&result, 0, &rule.reverse());
        assert_eq!(s, reverse_result);
    }

    #[test]
    fn part1_example() {
        let s = b"H => HO
H => OH
O => HH

HOH
";
        assert_eq!("4", Day19::new().solve(s).0);
    }

    #[test]
    fn part2_example_1() {
        let s = b"e => H
e => O
H => HO
H => OH
O => HH

HOH
";
        assert_eq!("3", Day19::new().solve(s).1);
    }

    #[test]
    fn part2_example_2() {
        let s = b"e => H
e => O
H => HO
H => OH
O => HH

HOHOHO
";

        assert_eq!("6", Day19::new().solve(s).1);
    }

    #[test]
    fn part1_and_part2() {
        let mut d = Day19::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("535", answer.0);
        assert_eq!("212", answer.1);
    }
}
