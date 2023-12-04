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
    #[tabled(rename = "Output")]
    pub output: f64,
    #[tabled(rename = "Duration")]
    pub duration: u128,
}

impl Result {
    pub fn new(day: usize, name: String, part: bool, output: f64, duration: Duration) -> Self {
        Self {
            day,
            name,
            part,
            output,
            duration: duration.as_nanos(),
        }
    }
}
