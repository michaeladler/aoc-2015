use std::str;

use ahash::AHashMap;
use compact_str::CompactString;
use log::{debug, trace};

use crate::aoc_io::Solver;
use crate::parse;

#[derive(Debug)]
pub struct Day07 {}

impl Day07 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Day07 {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Operand {
    Value(u16),
    Reg(CompactString),
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Expression {
    Assignment(Operand),
    Not(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Lshift(Operand, Operand),
    Rshift(Operand, Operand),
}

fn parse_operand(a: &CompactString) -> Operand {
    if a.starts_with(|c| ('0'..='9').contains(&c)) {
        Operand::Value(a.parse().unwrap())
    } else {
        Operand::Reg(a.clone())
    }
}

fn eval(wires: &mut AHashMap<CompactString, Expression>, operand: &Operand) -> u16 {
    match operand {
        Operand::Value(val) => *val,
        Operand::Reg(wire) => {
            let rhs = wires[wire].clone();
            let val = match rhs {
                Expression::Assignment(op) => eval(wires, &op),
                Expression::Not(op) => !eval(wires, &op),
                Expression::And(op1, op2) => eval(wires, &op1) & eval(wires, &op2),
                Expression::Or(op1, op2) => eval(wires, &op1) | eval(wires, &op2),
                Expression::Lshift(op1, op2) => eval(wires, &op1) << eval(wires, &op2),
                Expression::Rshift(op1, op2) => eval(wires, &op1) >> eval(wires, &op2),
            };
            debug!("{} has value {}", wire, val);
            // this is crucial! without memoization, the computation is super slow!
            wires.insert(wire.clone(), Expression::Assignment(Operand::Value(val)));
            val
        }
    }
}

fn parse_program(input: &[u8]) -> AHashMap<CompactString, Expression> {
    let mut wires = AHashMap::new();
    let mut buf = input;
    let mut lhs: [CompactString; 3] = [
        CompactString::new(""),
        CompactString::new(""),
        CompactString::new(""),
    ];
    while !buf.is_empty() {
        trace!(
            "Parsing: {:?}",
            str::from_utf8(
                buf.iter()
                    .take_while(|&&b| b != b'\n')
                    .copied()
                    .collect::<Vec<u8>>()
                    .as_slice()
            )
            .unwrap()
        );
        let mut lhs_count = 0;
        while buf[0] != b'-' {
            let (rest, tk) = parse::alphanumeric(buf).expect("token of lhs");
            lhs[lhs_count] = tk;
            lhs_count += 1;
            buf = parse::skip_ws(rest);
        }

        let lhs = &lhs[..lhs_count];
        buf = &buf[3..];
        let (rest, rhs) = parse::token(buf).expect("rhs");
        debug!("{:?} -> {:?}", lhs, rhs);

        let value: Expression = match lhs_count {
            1 => Expression::Assignment(parse_operand(&lhs[0])),
            2 => {
                if lhs[0] == CompactString::new("NOT") {
                    Expression::Not(parse_operand(&lhs[1]))
                } else {
                    panic!("Unsupported unary operation");
                }
            }
            3 => {
                // binary operators
                if lhs[1] == CompactString::new("AND") {
                    Expression::And(parse_operand(&lhs[0]), parse_operand(&lhs[2]))
                } else if lhs[1] == CompactString::new("OR") {
                    Expression::Or(parse_operand(&lhs[0]), parse_operand(&lhs[2]))
                } else if lhs[1] == CompactString::new("LSHIFT") {
                    Expression::Lshift(parse_operand(&lhs[0]), parse_operand(&lhs[2]))
                } else if lhs[1] == CompactString::new("RSHIFT") {
                    Expression::Rshift(parse_operand(&lhs[0]), parse_operand(&lhs[2]))
                } else {
                    panic!("Unsupported unary operation");
                }
            }
            _ => {
                panic!("Internal error")
            }
        };
        wires.insert(rhs, value);

        buf = parse::seek_next_line(rest);
    }
    wires
}

impl Solver for Day07 {
    fn day(&self) -> i32 {
        7
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let mut wires = parse_program(input);
        let mut wires2 = wires.clone();
        let part1: u16 = eval(&mut wires, &Operand::Reg(CompactString::new("a")));
        wires2.insert(
            CompactString::new("b"),
            Expression::Assignment(Operand::Value(part1)),
        );
        let part2: u16 = eval(&mut wires2, &Operand::Reg(CompactString::new("a")));
        (part1.to_string(), part2.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_io;

    #[test]
    fn part1_example() {
        let s = r"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
";
        let mut wires = parse_program(s.as_bytes());
        assert_eq!(
            123,
            eval(&mut wires, &Operand::Reg(CompactString::new("x")))
        );
        assert_eq!(
            456,
            eval(&mut wires, &Operand::Reg(CompactString::new("y")))
        );
        assert_eq!(72, eval(&mut wires, &Operand::Reg(CompactString::new("d"))));
        assert_eq!(
            507,
            eval(&mut wires, &Operand::Reg(CompactString::new("e")))
        );
        assert_eq!(
            492,
            eval(&mut wires, &Operand::Reg(CompactString::new("f")))
        );
        assert_eq!(
            114,
            eval(&mut wires, &Operand::Reg(CompactString::new("g")))
        );
        assert_eq!(
            65412,
            eval(&mut wires, &Operand::Reg(CompactString::new("h")))
        );
        assert_eq!(
            65079,
            eval(&mut wires, &Operand::Reg(CompactString::new("i")))
        );
    }

    #[test]
    fn part1_and_part2() {
        let mut d = Day07::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("16076", answer.0);
        assert_eq!("2797", answer.1);
    }
}
