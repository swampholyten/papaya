use std::io::{self, stdout};
use std::time::Duration;

use crossterm::{event, execute, terminal};
use ratatui::prelude::*;
use ratatui::backend::CrosstermBackend;

mod app;
mod stats;
mod theme;
mod ui;
mod wordlist;

use app::{App, AppMode};
use theme::Theme;

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen, event::EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen, event::DisableMouseCapture)?;
    terminal.show_cursor()?;

    res
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
    let mut app = App::new();
    let theme = Theme::default();
    loop {
        if let AppMode::Typing = app.mode {
            if !app.test_finished {
                app.update_stats();
                if app.finished() {
                    app.update_stats();
                    app.test_finished = true;
                    app.mode = AppMode::Summary;
                }
            }
        }

        terminal.draw(|f| ui::draw(f, &app, &theme))?;

        if app.should_quit {
            break;
        }

        if event::poll(Duration::from_millis(50))? {
            if let event::Event::Key(key) = event::read()? {
                app.on_key(key);
            }
        }
    }
    Ok(())
}
