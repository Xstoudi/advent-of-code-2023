use std::collections::HashMap;
use crate::traits::solver::Solver;

pub struct Day04;

impl Day04 {

}

impl Solver for Day04 {
    fn day(&self) -> usize {
        4
    }

    fn name(&self) -> String {
        String::from("Scratchcards")
    }

    fn solve_first(&self) -> u128 {
        self.input_first()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| ScratchCard::from(line.to_string()))
            .map(ScratchCard::point_value)
            .sum::<u32>() as u128
    }

    fn solve_second(&self) -> u128 {
        let mut cards = HashMap::new();
        let initial_cards = self.input_second()
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| ScratchCard::from(line.to_string()))
            .collect::<Vec<ScratchCard>>();

        for card in &initial_cards {
            cards.insert(card.id, 1);
        }

        for card in initial_cards {
            for i in 1..card.winning_count() + 1 {
                let concerned_id = card.id + i;
                let current_count = *cards.get(&card.id).unwrap_or(&0);
                cards
                    .entry(concerned_id)
                    .and_modify(|count| *count += 1 * current_count)
                    .or_insert(1);
            }
        }

        cards
            .iter()
            .map(|(_, count)| *count)
            .sum::<u32>() as u128
    }

    fn input_first(&self) -> String {
        include_str!("../resource/day04a.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/day04b.txt").to_string()
    }
}

#[derive(Clone)]
struct ScratchCard {
    id: u32,
    winning_numbers: Vec<u8>,
    scratch_numbers: Vec<u8>,
}

impl ScratchCard {
    fn point_value(self) -> u32 {
        self.scratch_numbers
            .iter()
            .filter(|scratch_number| self.winning_numbers.contains(scratch_number))
            .count()
            .checked_sub(1)
            .map_or(0, |count| 2u32.pow((count) as u32))
    }

    fn winning_count(&self) -> u32 {
        self.scratch_numbers
            .iter()
            .filter(|scratch_number| self.winning_numbers.contains(scratch_number))
            .count() as u32
    }
}

impl From<String> for ScratchCard {
    fn from(value: String) -> Self {
        let mut parts = value.split(":");

        let id = parts
            .next()
            .expect("Missing part 1")
            .trim()
            .split(" ")
            .last()
            .expect("Missing id")
            .trim()
            .parse::<u32>()
            .expect("Id is not a number");

        let mut number_parts = parts
            .next()
            .expect("Missing part 2")
            .trim()
            .split("|");

        let winning_numbers = number_parts
            .next()
            .expect("Missing winning numbers")
            .trim()
            .split(" ")
            .filter(|number| !number.is_empty())
            .map(|number| number.parse::<u8>().expect("Winning number is not a number"))
            .collect::<Vec<u8>>();

        let scratch_numbers = number_parts
            .next()
            .expect("Missing scratch numbers")
            .trim()
            .split(" ")
            .filter(|number| !number.is_empty())
            .map(|number| number.parse::<u8>().expect("Scratch number is not a number"))
            .collect::<Vec<u8>>();

        ScratchCard {
            id,
            winning_numbers,
            scratch_numbers,
        }
    }
}