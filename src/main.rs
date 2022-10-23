use std::env;
use std::io::{self, Write};
use std::time::Instant;

use aoc2015::aoc_io::{self, Solver};

fn main() {
    env_logger::builder().format_timestamp(None).init();

    #[rustfmt::skip]
    let mut all_days: Vec<Box<dyn Solver>> = vec![
        Box::new(aoc2015::Day01::new()),
        Box::new(aoc2015::Day02::new()),
        Box::new(aoc2015::Day03::new()),
        Box::new(aoc2015::Day04::new()),
        Box::new(aoc2015::Day05::new()),
        Box::new(aoc2015::Day06::new()),
        Box::new(aoc2015::Day07::new()),
        Box::new(aoc2015::Day08::new()),
        Box::new(aoc2015::Day09::new()),
        Box::new(aoc2015::Day10::new()),
        Box::new(aoc2015::Day11::new()),
        Box::new(aoc2015::Day12::new()),
        Box::new(aoc2015::Day13::new()),
        Box::new(aoc2015::Day14::new()),
        Box::new(aoc2015::Day15::new()),
        Box::new(aoc2015::Day16::new()),
        Box::new(aoc2015::Day17::new()),
        Box::new(aoc2015::Day18::new()),
        Box::new(aoc2015::Day19::new()),
        Box::new(aoc2015::Day20::new()),
        Box::new(aoc2015::Day21::new()),
        Box::new(aoc2015::Day22::new()),
        Box::new(aoc2015::Day23::new()),
        Box::new(aoc2015::Day24::new()),
        Box::new(aoc2015::Day25::new()),
        // marker
    ];

    let mut indices = Vec::new();

    for arg in env::args().skip(1) {
        let pos = arg.trim_start_matches('0').parse::<usize>().unwrap() - 1;
        indices.push(pos);
    }
    if indices.is_empty() {
        indices = (0..all_days.len()).collect();
    }

    let out = io::stdout();
    let mut handle = out.lock();

    let mut total_ms: f64 = 0.0;

    for idx in indices {
        let day = &mut all_days[idx];
        let input = aoc_io::read_input(day.day()).unwrap();
        let now = Instant::now();
        let output = day.solve(&input);
        let duration = now.elapsed().as_nanos() as u64;
        let duration_ms: f64 = duration as f64 / 1_000_000.;
        total_ms += duration_ms;

        writeln!(
            handle,
            "[Day {:02}]\tpart1: {:<16}\tpart2: {:<16}\tduration: {:>10.3} ms",
            day.day(),
            output.0,
            output.1,
            duration_ms
        )
        .unwrap();
    }

    writeln!(handle, "\nTotal: {:.3} ms", total_ms).unwrap();
}
