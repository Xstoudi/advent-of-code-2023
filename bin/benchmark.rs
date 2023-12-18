use std::iter::repeat;
use std::time::Duration;
use colored::Colorize;
use itertools::Itertools;
use tabled::settings::{Alignment, Color, Format, Merge, Modify, Panel, Style};
use tabled::settings::object::{Columns, Object, Rows};
use tabled::Table;
use aoc_2023::calendar::jobs;
use aoc_2023::structs::bench_result::BenchResult;
use rayon::prelude::*;

const REPEAT: usize = 100;

fn main() {
    rayon::ThreadPoolBuilder::new().build_global().unwrap();

    let jobs = jobs::get();
    let length = jobs.len();

    let results = (0..length)
        .map(|i| jobs[i % length])
        .collect::<Vec<_>>()
        .par_iter()
        .map(|job|
            repeat(job)
                .take(REPEAT)
                .flat_map(|job| job.run())
        )
        .flatten_iter()
        .collect::<Vec<_>>()
        .iter()
        .into_group_map_by(|r| format!("{:0>2}{}{}", r.day, r.name.clone(), r.part))
        .into_iter()
        .sorted_by_key(|(name, _)| name.clone())
        .map(|x| BenchResult::from(x))
        .collect::<Vec<_>>();


    let mut table = Table::new(&results);
    table
        .with(Panel::header("🔥 Benchmarked Advent of Code 2023 🔥"))
        .with(Panel::footer(format!("{} / 24", results.len() / 2)))
        .with(Style::modern())
        .with(Merge::vertical())
        .with(Alignment::center_vertical())
        .with(
            Modify::new(Rows::first().and(Rows::last()))
                .with(Alignment::center())
                .with(Color::BOLD)
        )
        .with(
            Modify::new(Columns::single(0).not(Rows::new(..2)).not(Rows::last()))
                .with(Alignment::center())
                .with(Color::BOLD)
        )
        .with(
            Modify::new(Columns::single(2).not(Rows::new(..2)).not(Rows::last()))
                .with(Format::content(|content|
                    if content == "false" {
                        "1".to_string()
                    } else {
                        "2".to_string()
                    }
                ))
                .with(Alignment::center())
        )
        .with(
            Modify::new(Columns::new(3..6).not(Rows::new(..2)).not(Rows::last()))
                .with(Format::content(|content| {
                    let nanos = content.parse::<u64>().expect("Failed to parse nanos");
                    let elapsed = Duration::from_nanos(nanos);
                    let secs = u128::from(elapsed.as_secs());
                    let millis = elapsed.as_millis();
                    let micros = elapsed.as_micros();
                    let nanos = elapsed.as_nanos();

                    let (major, minor, t) = if secs > 0 {
                        (secs, millis, "s")
                    } else if millis > 0 {
                        (millis, micros, "ms")
                    } else if micros > 0 {
                        (micros, nanos, "μs")
                    } else {
                        (nanos, nanos * 1000, "ns")
                    };

                    let time = major + (minor - major * 1000) / 1000;
                    let text = format!("{:.2} {}", time, t);

                    return if elapsed.as_secs() > 1 { text.red().to_string() } else { text.green().to_string() };
                }))
                .with(Alignment::right())
        );
    println!("{}", table);
}