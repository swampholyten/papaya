# Papaya

Papaya is a minimal yet extensible typing application for the terminal inspired by [MonkeyType](https://monkeytype.com/). It is built with the [`ratatui`](https://crates.io/crates/ratatui) TUI library and serves as a starting point for adding features like WPM tracking, persistent user statistics, custom themes, and word lists.

## Usage

```bash
cargo run --release
```

Press <kbd>Esc</kbd> to quit. Typing progress and WPM are displayed at the top of the interface.

## Architecture

- **`app.rs`** – application state and input handling
- **`ui.rs`** – rendering logic based on `ratatui`
- **`wordlist.rs`** – generates the words to type (can be extended for custom lists)
- **`stats.rs`** – utilities for computing typing statistics
- **`theme.rs`** – defines color themes

The code is intentionally small to keep it easy to extend.
