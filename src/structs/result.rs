use std::time::Duration;
use tabled::Tabled;

#[derive(Tabled)]
pub struct Result {
    #[tabled(rename = "Day")]
    pub day: usize,
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "Part")]
    pub part: bool,
    #[tabled(rename = "Answer")]
    pub output: u128,
    #[tabled(rename = "Compute time")]
    pub duration: u128,
}

impl Result {
    pub fn new(day: usize, name: String, part: bool, output: u128, duration: Duration) -> Self {
        Self {
            day,
            name,
            part,
            output,
            duration: duration.as_nanos(),
        }
    }
}
