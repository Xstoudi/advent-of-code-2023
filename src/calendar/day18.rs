use itertools::Itertools;
use crate::traits::solver::Solver;

pub struct Day18;

impl Day18 {
    fn char_to_operation(c: char) -> Operation {
        match c {
            'R' => Operation::Right,
            'L' => Operation::Left,
            'U' => Operation::Up,
            'D' => Operation::Down,
            _ => panic!("Invalid operation char: {}", c),
        }
    }

    fn digit_to_operation(c: char) -> Operation {
        match c {
            '0' => Operation::Right,
            '1' => Operation::Down,
            '2' => Operation::Left,
            '3' => Operation::Up,
            _ => panic!("Invalid operation char: {}", c),
        }
    }

    fn compute_area(state: &State) -> i64 {
        let area = (0..state.border.len() - 1)
            .fold(0, |acc, i| {
                let a = state.border[i];
                let b = state.border[i + 1];
                acc + a.0 * b.1 - a.1 * b.0
            })
            .abs()
            .div_euclid(2);

        let inside_area = area - state.perimeter.div_euclid(2) + 1;

        inside_area + state.perimeter
    }
}

impl Solver for Day18 {
    fn day(&self) -> usize {
        18
    }

    fn name(&self) -> String {
        String::from("Lavaduct Lagoon")
    }

    fn solve_first(&self) -> u128 {
        let state = self.input_first()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.split(" ").collect::<Vec<&str>>())
            .map(|part| Instruction::new(
                Day18::char_to_operation(
                    part
                        .iter()
                        .nth(0)
                        .expect("Operation not found")
                        .chars()
                        .next()
                        .expect("Operation char not found")
                ),
                part
                    .iter()
                    .nth(1)
                    .expect("Size not found")
                    .parse::<i64>()
                    .expect("Size is not a number"),
            ))
            .fold(State::default(), |mut state, instruction| {
                state.border.push(state.position);
                state.perimeter += instruction.size;
                let movement = match instruction.operation {
                    Operation::Right => (0, 1),
                    Operation::Left => (0, -1),
                    Operation::Up => (-1, 0),
                    Operation::Down => (1, 0),
                };
                state.position = (state.position.0 + movement.0 * instruction.size, state.position.1 + movement.1 * instruction.size);
                state
            });

        Day18::compute_area(&state) as u128
    }

    fn solve_second(&self) -> u128 {
        let state = self.input_second()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.split(" ").collect::<Vec<&str>>())
            .map(|parts| Instruction::new(
                Day18::digit_to_operation(
                    parts
                        .iter()
                        .nth(2)
                        .expect("Operation not found")
                        .chars()
                        .skip(1)
                        .skip(1)
                        .skip(5)
                        .next()
                        .expect("Operation char not found")
                ),
                i64::from_str_radix(parts
                    .iter()
                    .nth(2)
                    .expect("Size not found")
                    .chars()
                    .skip(1)
                    .skip(1)
                    .take(5)
                    .join("")
                    .as_str(),
                    16
                ).expect("Size is not an hex number")
            ))
            .fold(State::default(), |mut state, instruction| {
                state.border.push(state.position);
                state.perimeter += instruction.size;
                let movement = match instruction.operation {
                    Operation::Right => (0, 1),
                    Operation::Left => (0, -1),
                    Operation::Up => (-1, 0),
                    Operation::Down => (1, 0),
                };
                state.position = (state.position.0 + movement.0 * instruction.size, state.position.1 + movement.1 * instruction.size);
                state
            });

        Day18::compute_area(&state) as u128
    }

    fn input_first(&self) -> String {
        include_str!("../resource/day18.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/day18.txt").to_string()
    }
}

enum Operation {
    Right,
    Left,
    Up,
    Down,
}

struct State {
    position: (i64, i64),
    border: Vec<(i64, i64)>,
    perimeter: i64,
}

impl State {
    fn default() -> State {
        State {
            position: (0, 0),
            border: Vec::new(),
            perimeter: 0,
        }
    }
}

struct Instruction {
    operation: Operation,
    size: i64,
}

impl Instruction {
    fn new(operation: Operation, size: i64) -> Instruction {
        Instruction {
            operation,
            size,
        }
    }
}
