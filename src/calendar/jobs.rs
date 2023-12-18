use crate::calendar::day01::Day01;
use crate::calendar::day02::Day02;
use crate::calendar::day03::Day03;
use crate::calendar::day04::Day04;
use crate::calendar::day05::Day05;
use crate::calendar::day06::Day06;
use crate::calendar::day07::Day07;
use crate::calendar::day08::Day08;
use crate::calendar::day09::Day09;
use crate::calendar::day10::Day10;
use crate::calendar::day11::Day11;
use crate::calendar::day12::Day12;
use crate::calendar::day13::Day13;
use crate::calendar::day14::Day14;
use crate::calendar::day15::Day15;
use crate::calendar::day16::Day16;
use crate::calendar::day17::Day17;
use crate::calendar::day18::Day18;
use crate::traits::solver::Solver;

pub fn get() -> Vec<&'static dyn Solver> {
    vec![
        &Day01,
        &Day02,
        &Day03,
        &Day04,
        &Day05,
        &Day06,
        &Day07,
        &Day08,
        &Day09,
        &Day10,
        &Day11,
        &Day12,
        &Day13,
        &Day14,
        &Day15,
        &Day16,
        &Day17,
        &Day18
    ]
}

