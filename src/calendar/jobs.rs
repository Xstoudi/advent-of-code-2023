use crate::calendar::day01::Day01;
use crate::calendar::day02::Day02;
use crate::calendar::day03::Day03;
use crate::traits::solver::Solver;

pub fn get() -> Vec<&'static dyn Solver> {
    vec![
        &Day01,
        &Day02,
        &Day03
    ]
}

