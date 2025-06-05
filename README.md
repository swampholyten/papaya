# Papaya

Papaya is a minimal yet extensible typing application for the terminal inspired by [MonkeyType](https://monkeytype.com/). It is built with the [`ratatui`](https://crates.io/crates/ratatui) TUI library and serves as a starting point for adding features like WPM tracking, persistent user statistics, custom themes, and word lists.

## Usage

```bash
cargo run --release
```

When launched you can select a test length from the menu (30s, 60s or 120s)
using the arrow keys or `j`/`k`. Press Enter to start. During the test an
underlined cursor highlights the current character while correct, incorrect and
pending text are styled using a Catppuccin inspired palette. Time remaining and
live WPM appear at the top. After finishing, a centered results table shows your
time, WPM, accuracy and mistake count. You can then restart, return to the menu
or quit.

## Architecture

- **`app.rs`** – application state and input handling
- **`ui.rs`** – rendering logic based on `ratatui`
- **`wordlist.rs`** – generates the words to type (can be extended for custom lists)
- **`stats.rs`** – utilities for computing typing statistics
Papaya ships with a default theme based on the
[Catppuccin Mocha](https://catppuccin.com/palette/mocha) palette to provide
pleasant contrast across light and dark terminals.

- **`theme.rs`** – defines color themes