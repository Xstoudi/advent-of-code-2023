use std::time::Duration;
use tabled::Tabled;
use crate::structs::bench_result::BenchResult;

#[derive(Tabled)]
pub struct BenchRow {
    pub name: String,
    pub min_duration: String,
    pub avg_duration: String,
    pub max_duration: String,
}

impl From<BenchResult> for BenchRow {
    fn from(result: BenchResult) -> Self {
        Self {
            name: result.name,
            min_duration: fmt(result.min_duration),
            avg_duration: fmt(result.avg_duration),
            max_duration: fmt(result.max_duration),
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

