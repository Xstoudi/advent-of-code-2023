use std::time::Duration;
use colored::{Colorize};
use tabled::{
    Table,
    settings::{Modify, format::Format, object::{Rows, Columns, Object}, Style, Alignment},
};
use tabled::settings::{Color, Merge, Panel};
use aoc_2023::calendar::jobs;

fn main() {
    let results = jobs::get()
        .iter()
        .flat_map(|job| job.run())
        .collect::<Vec<_>>();

    let mut table = Table::new(&results);
    table
        .with(Panel::header("🎄 Advent of Code 2023 🎄"))
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
            Modify::new(Columns::single(3).not(Rows::new(..2)).not(Rows::last()))
                .with(Alignment::right())
        )
        .with(
            Modify::new(Columns::single(4).not(Rows::new(..2)).not(Rows::last()))
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

                    let time = major as f64 + (minor - major * 1000) as f64 / 1000.0;
                    let text = format!("{:.2} {}", time, t);

                    return if elapsed.as_secs() > 1 { text.red().to_string() } else { text.green().to_string() }
                }))
                .with(Alignment::right())
        );
    println!("{table}");
}