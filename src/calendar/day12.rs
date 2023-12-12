use hashbrown::HashMap;
use crate::traits::solver::Solver;

pub struct Day12;

impl Day12 {
}

impl Solver for Day12 {
    fn day(&self) -> usize {
        12
    }

    fn name(&self) -> String {
        String::from("Hot Springs")
    }

    fn solve_first(&self) -> u128 {
        let mut hot_spring = HotSpring::new();

        self.input_first()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut parts = line.split(" ");
                let status = parts
                    .next()
                    .unwrap()
                    .to_string();
                let continuous_damaged = parts
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|part| part.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                (status, continuous_damaged)
            })
            .map(|x| hot_spring.count_permutations(x, 0))
            .sum()
    }

    fn solve_second(&self) -> u128 {
        let mut hot_spring = HotSpring::new();

        self.input_second()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut parts = line.split(" ");
                let status = parts
                    .next()
                    .unwrap()
                    .to_string();

                let unfolded_status = (0..5)
                    .map(|_| status.clone())
                    .collect::<Vec<String>>()
                    .join("?");

                let continuous_damaged = parts
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|part| part.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
                    .repeat(5);

                (unfolded_status, continuous_damaged)
            })
            .map(|x| hot_spring.count_permutations(x, 0))
            .sum()
    }

    fn input_first(&self) -> String {
        include_str!("../resource/day12a.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/day12b.txt").to_string()
    }
}

struct HotSpring {
    memoization: HashMap<(String, Vec<usize>), u128>,
}

impl HotSpring {
    fn new() -> HotSpring {
        HotSpring {
            memoization: HashMap::new(),
        }
    }

    fn count_permutations(&mut self, (condition, continuous_damaged): (String, Vec<usize>), tab_size: u32) -> u128 {
        let input = (condition.clone(), continuous_damaged.clone());
        if let Some(count) = self.memoization.get(&(input)) {
            return *count;
        }

        if condition.is_empty() {
            return if continuous_damaged.is_empty() { 1 } else { 0 }
        }

        let first_status = condition.chars().take(1).last().unwrap();

        let permutations = match first_status {
            '.' => {
                self.count_permutations((condition[1..].to_string(), continuous_damaged), tab_size + 1)
            },
            '?' => {
                self.count_permutations((
                    String::from('.') + &condition[1..],
                    continuous_damaged.clone()
                ), tab_size + 1) +
                self.count_permutations((
                    String::from('#') + &condition[1..],
                    continuous_damaged
                ), tab_size + 1)
            },
            '#' => {
                if continuous_damaged.is_empty() {
                    return 0;
                }

                let damaged_count = continuous_damaged.first().unwrap();
                if damaged_count <= &condition.len() && condition.chars().take(*damaged_count).all(|c| c == '#' || c == '?') {
                    let new_continous_damaged = &continuous_damaged[1..];
                    return if *damaged_count == condition.len() {
                        if new_continous_damaged.is_empty() { 1 } else { 0 }
                    } else if condition.chars().nth(*damaged_count).unwrap() == '.' {
                        self.count_permutations((
                            condition[*damaged_count + 1..].to_string(),
                            new_continous_damaged.to_vec()
                        ), tab_size + 1)
                    } else if condition.chars().nth(*damaged_count).unwrap() == '?' {
                        self.count_permutations((
                            String::from('.') + &condition[*damaged_count + 1..],
                            new_continous_damaged.to_vec()
                        ), tab_size + 1)
                    } else {
                        0
                    }
                }

                0
            }
            _ => panic!("Cannot count permutations on undetermined spring status")
        };

        self.memoization.insert(input, permutations);
        permutations
    }

}

