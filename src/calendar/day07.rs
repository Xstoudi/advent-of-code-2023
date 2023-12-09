use std::cmp::Ordering;
use std::collections::HashMap;
use itertools::Itertools;
use strum_macros::Display;
use crate::traits::solver::Solver;

pub struct Day07;

impl Day07 {
    fn char_to_card(&self, character: char, allow_joker: bool) -> Card {
        match character {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => if allow_joker { Card::Joker } else { Card::Jack },
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Unknown card {}", character)
        }
    }
    fn cards_to_hand_value(&self, cards: &Vec<Card>) -> HandValues {
        let mut groups: HashMap<&Card, Vec<&Card>> = cards
            .into_iter()
            .into_group_map_by(|x| *x);

        let joker_count = groups.get(&Card::Joker).map(|x| x.len()).unwrap_or(0);

        groups.remove(&Card::Joker);

        if joker_count == 5 || groups.values().any(|x| x.len() + joker_count == 5) {
            return HandValues::FiveOfAKind;
        }

        if groups.values().any(|x| x.len() + joker_count == 4) {
            return HandValues::FourOfAKind;
        }

        match joker_count {
            0 => {
                if groups.values().any(|x| x.len() == 3) && groups.values().any(|x| x.len() == 2) {
                    return HandValues::FullHouse;
                }
            }
            1 => {
                if groups.values().filter(|x| x.len() == 2).count() == 2 {
                    return HandValues::FullHouse;
                }
            }
            2 => {
                if groups.values().any(|x| x.len() == 1) && groups.values().any(|x| x.len() == 2) {
                    return HandValues::FullHouse;
                }
            }
            3 => {
                if groups.values().any(|x| x.len() == 2) {
                    return HandValues::FullHouse;
                }
            }
            _ => {}
        }

        if groups.values().any(|x| x.len() + joker_count == 3) {
            return HandValues::ThreeOfAKind;
        }

        if groups.values().filter(|x| x.len() + joker_count == 2).count() == 2 {
            return HandValues::TwoPair;
        }

        if groups.values().any(|x| x.len() + joker_count == 2) {
            return HandValues::OnePair;
        }


        if cards.iter().all_unique() {
            return HandValues::HighCard;
        }

        panic!("Unknown hand value for cards {:?}", cards)
    }

    fn solve(&self, input: String, allow_joker: bool) -> u128 {
        let x = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.split(" "))
            .map(|mut line| {
                let hand_chars = line.nth(0).expect("No hand found").trim().to_string();
                let bid = line.nth(0).expect("No bid found").parse::<u64>().expect("Bid is not a number");

                let cards = hand_chars
                    .chars()
                    .map(|character| self.char_to_card(character, allow_joker))
                    .collect::<Vec<Card>>();

                let hand_value = self.cards_to_hand_value(&cards);

                (
                    Hand {
                        chars: hand_chars,
                        cards,
                        value: hand_value,
                    },
                    bid
                )
            })
            .sorted_by_key(|x: &(Hand, u64)| x.0.clone())
            .enumerate()
            .map(|(i, (_, bid))| bid * (i + 1) as u64)
            .sum::<u64>() as u128;
        x
    }
}

impl Solver for Day07 {
    fn day(&self) -> usize {
        7
    }

    fn name(&self) -> String {
        String::from("Camel Cards")
    }

    fn solve_first(&self) -> u128 {
        self.solve(self.input_first(), false)
    }

    fn solve_second(&self) -> u128 {
        self.solve(self.input_second(), true)
    }

    fn input_first(&self) -> String {
        include_str!("../resource/day07a.txt").to_string()
    }

    fn input_second(&self) -> String {
        include_str!("../resource/day07b.txt").to_string()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Hand {
    chars: String,
    cards: Vec<Card>,
    value: HandValues,
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value > other.value {
            return Ordering::Less;
        }
        if self.value < other.value {
            return Ordering::Greater;
        }

        for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            if self_card > other_card {
                return Ordering::Less;
            }
            if self_card < other_card {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
enum HandValues {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

#[derive(Display, Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}
