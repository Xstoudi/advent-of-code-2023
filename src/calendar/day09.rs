use crate::traits::solver::Solver;

pub struct Day09;

impl Day09 {

}

impl Solver for Day09 {
    fn day(&self) -> usize {
        9
    }

    fn name(&self) -> String {
        String::from("Mirage Maintenance")
    }

    fn solve_first(&self) -> u128 {
        self.input_first()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line|
                line
                    .split(" ")
                    .collect::<Vec<&str>>()
            )
            .map(|numbers|
                numbers
                    .iter()
                    .map(|number|
                        number
                            .parse::<i128>()
                            .expect("Invalid number")
                    )
                    .collect::<Vec<i128>>()
            )
            .map(|numbers| {
                let mut history = History::new(false);
                history.set_first_level(numbers);
                history
            })
            .map(|mut history| {
                history.push_zero();
                history
            })
            .map(|history| history.get_last_value())
            .sum::<i128>() as u128
    }

    fn solve_second(&self) -> u128 {
        self.input_second()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line|
                line
                    .split(" ")
                    .filter(|number| !number.is_empty())
                    .collect::<Vec<&str>>()
            )
            .map(|numbers|
                numbers
                    .iter()
                    .map(|number|
                        number
                            .parse::<i128>()
                            .expect("Invalid number")
                    )
                    .collect::<Vec<i128>>()
            )
            .map(|numbers| {
                let mut history = History::new(true);
                history.set_first_level(numbers);
                history
            })
            .map(|mut history| {
                history.push_zero();
                history
            })
            .map(|history| history.get_last_value())
            .sum::<i128>() as u128
    }

    fn input_first(&self) -> String {
        include_str!("../resource/day09.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/day09.txt").to_string()
    }
}

struct History {
    numbers: Vec<Vec<i128>>,
    backward: bool
}

impl History {
    fn new(backward: bool) -> History {
        History {
            numbers: Vec::new(),
            backward
        }
    }

    fn set_first_level(&mut self, numbers: Vec<i128>) {
        self.numbers.push(numbers);
        self.compute_levels();
    }

    fn compute_levels(&mut self) {
        self.numbers.truncate(1);

        let mut current_level = 0;

        while !self.numbers
            .last()
            .unwrap()
            .iter()
            .all(|number| *number == 0) {

            let mut new_vec = Vec::new();
            for window in self.numbers.get(current_level).unwrap().windows(2) {
                let mut window_iter = window.iter();
                let first = *window_iter.next().unwrap(); // -446
                let second = *window_iter.next().unwrap(); // -500
                let delta = -(first - second);
                new_vec.push(delta);
            }
            self.numbers.push(new_vec);
            current_level += 1;
        }
    }

    fn push_zero(&mut self) {
        let mut new_numbers = self.numbers.clone();

        for pair in self.numbers.iter().enumerate().collect::<Vec<_>>().windows(2).rev() {
            let (first_index, first) = pair[0];
            let (second_index, _) = pair[1];

            if second_index == (self.numbers.len() - 1) {
                if self.backward {
                    new_numbers[second_index].insert(0, 0);
                } else {
                    new_numbers[second_index].push(0);
                }
            }

            if self.backward {
               let first_of_second = *new_numbers[second_index].first().unwrap();
                new_numbers[first_index].insert(
                     0,
                     -(first_of_second - first.first().unwrap())
                );
            } else {
                let last_of_second = *new_numbers[second_index].last().unwrap();
                new_numbers[first_index].push(
                    last_of_second + first.last().unwrap()
                );
            }
        }

        self.numbers = new_numbers;
    }

    fn get_last_value(&self) -> i128 {
        if self.backward {
            *self.numbers
                .first()
                .unwrap()
                .first()
                .unwrap()
        } else {
            *self.numbers
                .first()
                .unwrap()
                .last()
                .unwrap()
        }

    }
}