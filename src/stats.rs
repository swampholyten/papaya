use std::time::Duration;

#[derive(Default, Debug, Clone, Copy)]
pub struct Stats {
    pub wpm: f64,
    pub elapsed: Duration,
}

impl Stats {
    pub fn calculate_wpm(chars: usize, elapsed: Duration) -> f64 {
        if elapsed.as_secs_f64() == 0.0 {
            return 0.0;
        }
        let words = chars as f64 / 5.0;
        words / (elapsed.as_secs_f64() / 60.0)
    }
}
