use std::time::Duration;

#[derive(Default, Debug, Clone, Copy)]
pub struct Stats {
    pub wpm: f64,
    pub elapsed: Duration,
    pub mistakes: usize,
    pub chars: usize,
}

impl Stats {
    pub fn calculate_wpm(chars: usize, elapsed: Duration) -> f64 {
        if elapsed.as_secs_f64() == 0.0 {
            return 0.0;
        }
        let words = chars as f64 / 5.0;
        words / (elapsed.as_secs_f64() / 60.0)
    }

    pub fn count_mistakes(target: &str, typed: &str) -> usize {
        let mut mistakes = 0;
        for (i, tc) in typed.chars().enumerate() {
            if let Some(gc) = target.chars().nth(i) {
                if tc != gc {
                    mistakes += 1;
                }
            } else {
                mistakes += 1;
            }
        }
        mistakes
    }

    pub fn accuracy(&self) -> f64 {
        if self.chars == 0 {
            100.0
        } else {
            100.0 * (self.chars.saturating_sub(self.mistakes) as f64) / self.chars as f64
        }
    }
}
