use std::fs::{self, File};
use std::io;
use std::io::prelude::*;

pub trait Solver {
    fn day(&self) -> i32;
    fn solve(&mut self, input: &[u8]) -> (String, String);
}

pub fn read_input(day: i32) -> io::Result<Vec<u8>> {
    let fname = format!("input/day{:02}.txt", day);
    let mut file = File::open(&fname).unwrap();
    let len = fs::metadata(&fname).unwrap().len() as usize;
    let mut buf = Vec::with_capacity(len);
    file.read_to_end(&mut buf)?;
    Ok(buf)
}
