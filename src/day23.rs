use arrayvec::ArrayVec;
use log::debug;

use crate::aoc_io::Solver;
use crate::parse;

#[derive(Debug)]
pub struct Day23 {}

#[allow(clippy::new_without_default)]
impl Day23 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Reg {
    A,
    B,
}

impl Reg {
    pub const fn ordinal(&self) -> usize {
        match self {
            Reg::A => 0,
            Reg::B => 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Instruction {
    Hlf(Reg),
    Tpl(Reg),
    Inc(Reg),
    Jmp(i32),
    Jie(Reg, i32),
    Jio(Reg, i32),
}

fn parse_register(s: &[u8]) -> (&[u8], Reg) {
    let (rest, reg) = parse::token(s).unwrap();
    match reg.as_bytes() {
        b"a" => (rest, Reg::A),
        b"b" => (rest, Reg::B),
        _ => panic!("cannot parse register"),
    }
}

impl Solver for Day23 {
    fn day(&self) -> i32 {
        23
    }

    fn solve(&mut self, input: &[u8]) -> (String, String) {
        let mut prog = Prog::parse(input);
        let mut prog2 = prog.clone();

        prog.run();

        prog2.registers[Reg::A.ordinal()] = 1;
        prog2.run();

        let part1: i32 = prog.reg_val(Reg::B);
        let part2: i32 = prog2.reg_val(Reg::B);

        (part1.to_string(), part2.to_string())
    }
}

#[derive(Clone, Debug)]
struct Prog {
    code: ArrayVec<Instruction, 64>,
    registers: [i32; 2],
    ip: i32,
}

impl Prog {
    pub fn reg_val(&self, reg: Reg) -> i32 {
        unsafe { *self.registers.get_unchecked(reg.ordinal()) }
    }

    pub fn run(&mut self) {
        let n = self.code.len() as i32;
        while self.ip >= 0 && self.ip < n {
            let inst = unsafe { self.code.get_unchecked(self.ip as usize) };
            match inst {
                Instruction::Hlf(reg) => {
                    let elem = unsafe { self.registers.get_unchecked_mut(reg.ordinal()) };
                    *elem /= 2;
                    self.ip += 1;
                }
                Instruction::Tpl(reg) => {
                    let elem = unsafe { self.registers.get_unchecked_mut(reg.ordinal()) };
                    *elem *= 3;
                    self.ip += 1;
                }
                Instruction::Inc(reg) => {
                    let elem = unsafe { self.registers.get_unchecked_mut(reg.ordinal()) };
                    *elem += 1;
                    self.ip += 1;
                }
                Instruction::Jmp(offset) => {
                    self.ip += offset;
                }
                Instruction::Jie(reg, offset) => {
                    let elem = unsafe { self.registers.get_unchecked(reg.ordinal()) };
                    if *elem % 2 == 0 {
                        self.ip += offset;
                    } else {
                        self.ip += 1;
                    }
                }
                Instruction::Jio(reg, offset) => {
                    let elem = unsafe { self.registers.get_unchecked(reg.ordinal()) };
                    if *elem == 1 {
                        self.ip += offset;
                    } else {
                        self.ip += 1;
                    }
                }
            }
        }
    }

    pub fn parse(input: &[u8]) -> Self {
        let mut instructions: ArrayVec<Instruction, 64> = ArrayVec::new();

        let mut buf = input;
        while !buf.is_empty() {
            let (rest, token) = parse::token(buf).unwrap();
            let (rest, inst) = match token.as_bytes() {
                b"hlf" => {
                    let rest = parse::skip_ws(rest);
                    let (rest, reg) = parse_register(rest);
                    (rest, Instruction::Hlf(reg))
                }
                b"tpl" => {
                    let rest = parse::skip_ws(rest);
                    let (rest, reg) = parse_register(rest);
                    (rest, Instruction::Tpl(reg))
                }
                b"inc" => {
                    let rest = parse::skip_ws(rest);
                    let (rest, reg) = parse_register(rest);
                    (rest, Instruction::Inc(reg))
                }
                b"jmp" => {
                    let (rest, offset) = parse::integer(rest, true).unwrap();
                    (rest, Instruction::Jmp(offset as i32))
                }
                b"jie" => {
                    let rest = parse::skip_ws(rest);
                    let (rest, reg) = parse_register(rest);
                    let (rest, offset) = parse::integer(rest, true).unwrap();
                    (rest, Instruction::Jie(reg, offset as i32))
                }
                b"jio" => {
                    let rest = parse::skip_ws(rest);
                    let (rest, reg) = parse_register(rest);
                    let (rest, offset) = parse::integer(rest, true).unwrap();
                    (rest, Instruction::Jio(reg, offset as i32))
                }
                _ => panic!("unexpected token"),
            };
            debug!("parsed: {:?}", inst);
            unsafe {
                instructions.push_unchecked(inst);
            }

            buf = parse::seek_next_line(rest);
        }
        Prog {
            code: instructions,
            ip: 0,
            registers: [0, 0],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_io;

    #[test]
    fn example() {
        let mut prog = Prog::parse(
            b"inc a
jio a, +2
tpl a
inc a",
        );
        prog.run();
        assert_eq!(2, prog.reg_val(Reg::A));
    }

    #[test]
    fn part1_and_part2() {
        let mut d = Day23::new();
        let answer = d.solve(&aoc_io::read_input(d.day()).unwrap());
        assert_eq!("255", answer.0);
        assert_eq!("334", answer.1);
    }
}
