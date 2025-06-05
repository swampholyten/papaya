use std::time::Instant;
use crossterm::event::KeyEvent;

use crate::stats::Stats;
use crate::wordlist;

pub struct App {
    pub target: String,
    pub typed: String,
    pub started: Option<Instant>,
    pub stats: Stats,
}

impl App {
    pub fn new() -> Self {
        let words = wordlist::random_words(25);
        let target = words.join(" ");
        Self {
            target,
            typed: String::new(),
            started: None,
            stats: Stats::default(),
        }
    }

    pub fn on_key(&mut self, event: KeyEvent) {
        use crossterm::event::{KeyCode, KeyModifiers};
        match event.code {
            KeyCode::Char(c) => {
                if event.modifiers.contains(KeyModifiers::CONTROL) {
                    return;
                }
                if self.started.is_none() {
                    self.started = Some(Instant::now());
                }
                if self.typed.len() < self.target.len() {
                    self.typed.push(c);
                }
            }
            KeyCode::Backspace => {
                self.typed.pop();
            }
            _ => {}
        }
    }

    pub fn finished(&self) -> bool {
        self.typed.len() >= self.target.len()
    }

    pub fn update_stats(&mut self) {
        if let Some(start) = self.started {
            let elapsed = start.elapsed();
            self.stats.elapsed = elapsed;
            self.stats.wpm = Stats::calculate_wpm(self.typed.len(), elapsed);
        }
    }
}
