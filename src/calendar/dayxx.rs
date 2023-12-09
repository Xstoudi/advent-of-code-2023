use crate::traits::solver::Solver;

pub struct DayXX;

impl DayXX {

}

impl Solver for DayXX {
    fn day(&self) -> usize {
        1
    }

    fn name(&self) -> String {
        String::from("Gear Ratios")
    }

    fn solve_first(&self) -> u128 {
        0.0
    }

    fn solve_second(&self) -> u128 {
        0.0
    }

    fn input_first(&self) -> String {
        include_str!("../resource/dayXXa.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/dayXXb.txt").to_string()
    }
}
