use ratatui::style::Color;

#[derive(Clone)]
pub struct Theme {
    pub correct: Color,
    pub incorrect: Color,
    pub pending: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            correct: Color::Green,
            incorrect: Color::Red,
            pending: Color::White,
        }
    }
}
