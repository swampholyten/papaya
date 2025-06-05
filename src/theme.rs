use ratatui::style::Color;

#[derive(Clone)]
pub struct Theme {
    pub background: Color,
    pub text: Color,
    pub accent: Color,
    pub correct: Color,
    pub incorrect: Color,
    pub pending: Color,
    pub current: Color,
    pub table_bg: Color,
    pub table_border: Color,
    pub menu_highlight: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::Rgb(30, 30, 46),      // Catppuccin base
            text: Color::Rgb(205, 214, 244),         // Catppuccin text
            accent: Color::Rgb(137, 180, 250),       // Blue
            correct: Color::Rgb(166, 227, 161),
            incorrect: Color::Rgb(243, 139, 168),
            pending: Color::Rgb(108, 112, 134),
            current: Color::Rgb(249, 226, 175),
            table_bg: Color::Rgb(49, 50, 68),
            table_border: Color::Rgb(108, 112, 134),
            menu_highlight: Color::Rgb(137, 180, 250),
        }
    }
}
