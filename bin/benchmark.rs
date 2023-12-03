use std::iter::repeat;
use itertools::Itertools;
use tabled::Table;
use aoc_2023::calendar::jobs;
use aoc_2023::structs::bench_result::BenchResult;
use aoc_2023::structs::bench_row::BenchRow;

fn main() {
    let results = jobs::get()
        .iter()
        .enumerate()
        .flat_map(|(i, &job)| {
            repeat(job)
                .take(100)
                .flat_map(move |job| job.run(i))
        })
        .collect::<Vec<_>>()
        .iter()
        .into_group_map_by(|r| r.name.clone())
        .into_iter()
        .sorted_by_key(|(name, _)| name.clone())
        .map(|x| BenchResult::from(x))
        .map(|result| BenchRow::from(result))
        .collect::<Vec<_>>();
    let table = Table::new(results);
    println!("{}", table);
}