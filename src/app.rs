use std::time::{Duration, Instant};
use crossterm::event::KeyEvent;

use crate::stats::Stats;
use crate::wordlist;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Menu,
    Typing,
    Summary,
}

pub struct App {
    pub mode: AppMode,
    pub target: String,
    pub typed: String,
    pub started: Option<Instant>,
    pub duration: Duration,
    pub durations: Vec<u64>,
    pub selected: usize,
    pub should_quit: bool,
    pub stats: Stats,
}

impl App {
    pub fn new() -> Self {
        Self {
            mode: AppMode::Menu,
            target: String::new(),
            typed: String::new(),
            started: None,
            duration: Duration::from_secs(60),
            durations: vec![30, 60, 120],
            selected: 1,
            should_quit: false,
            stats: Stats::default(),
        }
    }

    pub fn on_key(&mut self, event: KeyEvent) {
        match self.mode {
            AppMode::Menu => self.on_key_menu(event),
            AppMode::Typing => self.on_key_typing(event),
            AppMode::Summary => self.on_key_summary(event),
        }
    }

    fn on_key_menu(&mut self, event: KeyEvent) {
        use crossterm::event::KeyCode;
        match event.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.selected + 1 < self.durations.len() {
                    self.selected += 1;
                }
            }
            KeyCode::Enter => {
                self.duration = Duration::from_secs(self.durations[self.selected]);
                self.start_test();
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    fn on_key_typing(&mut self, event: KeyEvent) {
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
            KeyCode::Esc => self.should_quit = true,
            _ => {}
        }
    }

    fn on_key_summary(&mut self, event: KeyEvent) {
        use crossterm::event::KeyCode;
        match event.code {
            KeyCode::Char('r') => self.start_test(),
            KeyCode::Char('m') => self.reset_menu(),
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            _ => {}
        }
    }

    fn start_test(&mut self) {
        let words = wordlist::random_words(50);
        self.target = words.join(" ");
        self.typed.clear();
        self.started = None;
        self.stats = Stats::default();
        self.mode = AppMode::Typing;
    }

    fn reset_menu(&mut self) {
        self.mode = AppMode::Menu;
        self.started = None;
        self.typed.clear();
    }

    pub fn finished(&self) -> bool {
        if let Some(start) = self.started {
            start.elapsed() >= self.duration || self.typed.len() >= self.target.len()
        } else {
            false
        }
    }

    pub fn time_left(&self) -> Duration {
        if let Some(start) = self.started {
            if self.duration > start.elapsed() {
                self.duration - start.elapsed()
            } else {
                Duration::from_secs(0)
            }
        } else {
            self.duration
        }
    }

    pub fn update_stats(&mut self) {
        if let Some(start) = self.started {
            let elapsed = start.elapsed();
            self.stats.elapsed = elapsed.min(self.duration);
            self.stats.wpm = Stats::calculate_wpm(self.typed.len(), self.stats.elapsed);
            self.stats.mistakes = Stats::count_mistakes(&self.target, &self.typed);
            self.stats.chars = self.typed.len();
        }
    }
}
