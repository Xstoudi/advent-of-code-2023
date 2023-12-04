use std::ops::Div;
use tabled::Tabled;
use crate::structs::result::Result;

#[derive(Tabled)]
pub struct BenchResult {
    #[tabled(rename = "Day")]
    pub day: usize,
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "Part")]
    pub part: bool,
    #[tabled(rename = "Min duration")]
    pub min_duration: u128,
    #[tabled(rename = "Average duration")]
    pub avg_duration: u128,
    #[tabled(rename = "Max duration")]
    pub max_duration: u128,
}

impl BenchResult {
    pub fn new(day: usize, name: String, part: bool, min_duration: u128, avg_duration: u128, max_duration: u128) -> Self {
        Self {
            day,
            name,
            part,
            min_duration,
            avg_duration,
            max_duration,
        }
    }
}

impl From<(String, Vec<&Result>)> for BenchResult {
    fn from(from: (String, Vec<&Result>)) -> Self {
        let durations = from.1
            .iter()
            .map(|r| r.duration)
            .collect::<Vec<_>>();

        let min = durations
            .iter()
            .copied()
            .min()
            .unwrap_or(0);

        let avg = durations
            .iter()
            .copied()
            .sum::<u128>();

        let max = durations
            .iter()
            .copied()
            .max()
            .unwrap_or(0);

        Self {
            day: from.1[0].day,
            name: from.1[0].name.clone(),
            part: from.1[0].part,
            min_duration: min,
            avg_duration: avg.div(durations.len() as u128),
            max_duration: max,
        }
    }
}
