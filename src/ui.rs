use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::{App, AppMode, Theme};

pub fn draw(f: &mut Frame, app: &App, theme: &Theme) {
    match app.mode {
        AppMode::Menu => draw_menu(f, app),
        AppMode::Typing => draw_typing(f, app, theme),
        AppMode::Summary => draw_summary(f, app),
    }
}

fn build_text(app: &App, theme: &Theme) -> Text<'static> {
    let target_words: Vec<&str> = app.target.split_whitespace().collect();
    let mut typed_words: Vec<&str> = app.typed.split_whitespace().collect();
    let typed_ends_with_space = app.typed.ends_with(' ');

    let current_fragment = if typed_ends_with_space {
        ""
    } else {
        typed_words.pop().unwrap_or("")
    };

    let mut spans = Vec::new();
    for (i, word) in target_words.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled(" ", Style::default().fg(theme.pending)));
        }

        if i < typed_words.len() {
            let style = if typed_words[i] == *word {
                Style::default().fg(theme.correct)
            } else {
                Style::default().fg(theme.incorrect)
            };
            spans.push(Span::styled(word.to_string(), style));
        } else if i == typed_words.len() && !typed_ends_with_space {
            for (j, ch) in word.chars().enumerate() {
                let mut style = if let Some(tc) = current_fragment.chars().nth(j) {
                    if tc == ch {
                        Style::default().fg(theme.correct)
                    } else {
                        Style::default().fg(theme.incorrect)
                    }
                } else {
                    Style::default().fg(theme.pending)
                };
                if j == current_fragment.len() {
                    style = style.add_modifier(Modifier::UNDERLINED | Modifier::BOLD);
                }
                spans.push(Span::styled(ch.to_string(), style));
            }
        } else {
            spans.push(Span::styled((*word).to_string(), Style::default().fg(theme.pending)));
        }
    }

    if typed_words.len() >= target_words.len() && typed_ends_with_space {
        let style = Style::default()
            .fg(theme.pending)
            .add_modifier(Modifier::UNDERLINED | Modifier::BOLD);
        spans.push(Span::styled(" ", style));
    }

    Text::from(Line::from(spans))
}

fn draw_menu(f: &mut Frame, app: &App) {
    let block = Block::default().borders(Borders::ALL).title("Papaya");
    let text = vec![
        Line::from("Select time (e.g. 30s) then press Enter:"),
        Line::from(format!(":{}", app.input)),
    ];
    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, f.size());
}

fn draw_typing(f: &mut Frame, app: &App, theme: &Theme) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .split(size);

    let stats = Paragraph::new(format!("WPM: {:.2}  Time: {}s", app.stats.wpm, app.time_left().as_secs()))
        .block(Block::default());
    f.render_widget(stats, chunks[0]);

    let text = build_text(app, theme);
    let block = Block::default().borders(Borders::ALL).title("Papaya");
    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: false });
    f.render_widget(paragraph, chunks[1]);
}

fn draw_summary(f: &mut Frame, app: &App) {
    let block = Block::default().borders(Borders::ALL).title("Results");
    let text = vec![
        Line::from(format!("Time: {}s", app.stats.elapsed.as_secs())),
        Line::from(format!("WPM: {:.2}", app.stats.wpm)),
        Line::from(format!("Mistakes: {}", app.stats.mistakes)),
        Line::from(""),
        Line::from("Press r to restart, m for menu, q to quit"),
    ];
    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, f.size());
}
