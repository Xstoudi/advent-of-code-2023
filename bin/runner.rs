use tabled::Table;
use aoc_2023::calendar::jobs;
use aoc_2023::structs::row::Row;

fn main() {
    let results = jobs::get()
        .iter()
        .enumerate()
        .flat_map(|(i, &job)| job.run(i))
        .map(|result| Row::from(result))
        .collect::<Vec<_>>();
    let table = Table::new(results);
    println!("{}", table);
}