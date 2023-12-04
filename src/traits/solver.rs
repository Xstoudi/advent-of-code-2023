use took::Timer;
use crate::structs::result::Result;

pub trait Solver: Sync {
    fn day(&self) -> usize;
    fn name(&self) -> String;
    fn run(&self) -> Vec<Result> {
        let first_timer = Timer::new();
        let first_result  = (self.solve_first(), first_timer.took().into_std());

        let second_timer = Timer::new();
        let second_result = (self.solve_second(), second_timer.took().into_std());

        vec![
            Result::new(self.day(), self.name(), false, first_result.0, first_result.1),
            Result::new(self.day(), self.name(), true, second_result.0, second_result.1),
        ]
    }
    fn solve_first(&self) -> f64;
    fn solve_second(&self) -> f64;

    fn input_first(&self) -> String;
    fn input_second(&self) -> String;
}
