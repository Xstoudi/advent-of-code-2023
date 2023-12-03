use std::time::Duration;
use crate::structs::result::Result;

pub struct BenchResult {
    pub name: String,
    pub min_duration: Duration,
    pub avg_duration: Duration,
    pub max_duration: Duration,
}

impl BenchResult {
    pub fn new(name: String, min_duration: Duration, avg_duration: Duration, max_duration: Duration) -> Self {
        Self {
            name,
            min_duration,
            avg_duration,
            max_duration,
        }
    }
}

impl From<(String, Vec<&Result>)> for BenchResult {
    fn from(from: (String, Vec<&Result>)) -> Self {
        let durations = from.1.iter().map(|r| r.duration).collect::<Vec<_>>();

        let min = durations
            .iter()
            .copied()
            .min()
            .unwrap_or(Duration::from_secs(0));

        let avg = durations
            .iter()
            .copied()
            .sum::<Duration>();

        let max = durations
            .iter()
            .copied()
            .max()
            .unwrap_or(Duration::from_secs(0));

        Self {
            name: from.0,
            min_duration: min,
            avg_duration: avg.div_f32(durations.len() as f32),
            max_duration: max,
        }
    }
}
