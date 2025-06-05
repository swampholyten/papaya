use ratatui::prelude::*;
use ratatui::widgets::{Paragraph, Block, Borders};

use crate::{App, Theme};

pub fn draw(f: &mut Frame, app: &App, theme: &Theme) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .split(size);

    let stats = Paragraph::new(format!("WPM: {:.2}", app.stats.wpm));
    f.render_widget(stats, chunks[0]);

    let text = build_text(app, theme);
    let block = Block::default().borders(Borders::ALL).title("Papaya");
    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, chunks[1]);
}

fn build_text(app: &App, theme: &Theme) -> Text<'static> {
    let mut spans = Vec::new();
    for (i, ch) in app.target.chars().enumerate() {
        let style = if let Some(c) = app.typed.chars().nth(i) {
            if c == ch {
                Style::default().fg(theme.correct)
            } else {
                Style::default().fg(theme.incorrect)
            }
        } else {
            Style::default().fg(theme.pending)
        };
        spans.push(Span::styled(ch.to_string(), style));
    }
    Text::from(Line::from(spans))
}
