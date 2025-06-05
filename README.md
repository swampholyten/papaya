# Papaya

Papaya is a minimal yet extensible typing application for the terminal inspired by [MonkeyType](https://monkeytype.com/). It is built with the [`ratatui`](https://crates.io/crates/ratatui) TUI library and serves as a starting point for adding features like WPM tracking, persistent user statistics, custom themes, and word lists.

## Usage

```bash
cargo run --release
```

When launched you will be asked to choose a test length such as `30s` or `60s`.
During the test a blinking underline indicates the current character and the
remaining time and live WPM are shown at the top. After finishing you can
restart, return to the menu or quit.

## Architecture

- **`app.rs`** – application state and input handling
- **`ui.rs`** – rendering logic based on `ratatui`
- **`wordlist.rs`** – generates the words to type (can be extended for custom lists)
- **`stats.rs`** – utilities for computing typing statistics
- **`theme.rs`** – defines color themes

The code is intentionally small to keep it easy to extend.
