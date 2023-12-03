use std::time::Duration;

pub struct Result {
    pub name: String,
    pub value: f64,
    pub duration: Duration,
}

impl Result {
    pub fn new(name: String, value: f64, duration: Duration) -> Self {
        Self {
            name,
            value,
            duration,
        }
    }
}
