use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap, Table, Row, Cell};

use crate::{App, AppMode, Theme};

pub fn draw(f: &mut Frame, app: &App, theme: &Theme) {
    match app.mode {
        AppMode::Menu => draw_menu(f, app, theme),
        AppMode::Typing => draw_typing(f, app, theme),
        AppMode::Summary => draw_summary(f, app),
    }
}

fn build_text(app: &App, theme: &Theme) -> Text<'static> {
    let mut spans = Vec::new();
    for (i, ch) in app.target.chars().enumerate() {
        let mut style = if let Some(c) = app.typed.chars().nth(i) {
            if c == ch {
                Style::default().fg(theme.correct)
            } else {
                Style::default().fg(theme.incorrect)
            }
        } else {
            Style::default().fg(theme.pending)
        };
        if i == app.typed.len() {
            style = style
                .fg(theme.current)
                .add_modifier(Modifier::UNDERLINED | Modifier::BOLD);
        }
        spans.push(Span::styled(ch.to_string(), style));
    }
    if app.typed.len() >= app.target.len() {
        let style = Style::default()
            .fg(theme.current)
            .add_modifier(Modifier::UNDERLINED | Modifier::BOLD);
        spans.push(Span::styled(" ", style));
    }
    Text::from(Line::from(spans))
}

fn draw_menu(f: &mut Frame, app: &App, theme: &Theme) {
    let items: Vec<ListItem> = app
        .durations
        .iter()
        .map(|d| ListItem::new(format!("{}s", d)))
        .collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Select duration"))
        .highlight_style(
            Style::default()
                .fg(theme.current)
                .add_modifier(Modifier::BOLD | Modifier::REVERSED),
        )
        .highlight_symbol("> ");
    let mut state = ListState::default();
    state.select(Some(app.selected));
    f.render_stateful_widget(list, f.size(), &mut state);
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

    let stats = Paragraph::new(format!("WPM: {}  Time: {}s", app.stats.wpm.round() as u64, app.time_left().as_secs()))
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
    let area = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(3), Constraint::Length(2)])
        .split(area);

    let rows = vec![
        Row::new(vec![
            Cell::from("Time Taken"),
            Cell::from(format!("{}s", app.stats.elapsed.as_secs())),
        ]).style(Style::default().bg(Color::DarkGray)),
        Row::new(vec![Cell::from("WPM"), Cell::from(format!("{}", app.stats.wpm.round() as u64))]),
        Row::new(vec![
            Cell::from("Accuracy"),
            Cell::from(format!("{:.0}%", app.stats.accuracy())),
        ]).style(Style::default().bg(Color::DarkGray)),
        Row::new(vec![
            Cell::from("Mistyped"),
            Cell::from(format!("{} words", app.stats.mistakes)),
        ]),
    ];

    let table = Table::new(rows, [Constraint::Percentage(50), Constraint::Percentage(50)])
        .header(Row::new(vec!["Metric", "Value"]).style(Style::default().add_modifier(Modifier::BOLD)))
        .block(Block::default().borders(Borders::ALL).title("Results"))
        .column_spacing(2);

    f.render_widget(table, chunks[0]);

    let help = Paragraph::new("Press r to restart, m for menu, q to quit")
        .block(Block::default());
    f.render_widget(help, chunks[1]);
}
