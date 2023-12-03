use std::time::Duration;
use tabled::Tabled;
use crate::structs::result::Result;

#[derive(Tabled)]
pub struct Row {
    pub name: String,
    pub value: f64,
    pub duration: String,
}

impl From<Result> for Row {
    fn from(result: Result) -> Self {
        Self {
            name: result.name,
            value: result.value,
            duration: fmt(result.duration),
        }
    }
}

fn fmt(duration: Duration) -> String {
    let elapsed = duration;
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
    format!("{:.2} {}", time, t)
}
