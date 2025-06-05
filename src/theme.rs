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
            background: Color::Rgb(26, 27, 38),      // Tokyonight bg
            text: Color::Rgb(192, 202, 245),         // Tokyonight fg
            accent: Color::Rgb(122, 162, 247),       // Blue
            correct: Color::Rgb(122, 162, 247),      // Blue for correct words
            incorrect: Color::Rgb(247, 118, 142),    // Red
            pending: Color::Rgb(86, 95, 137),        // Grey
            current: Color::Rgb(122, 162, 247),      // Blue for cursor
            table_bg: Color::Rgb(31, 35, 53),
            table_border: Color::Rgb(65, 72, 104),
            menu_highlight: Color::Rgb(122, 162, 247),
        }
    }
}
