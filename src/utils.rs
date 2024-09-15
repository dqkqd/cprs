use anyhow::Context;
use crossterm::{execute, style::Print};

pub fn println_to_console<S: std::fmt::Display>(s: S) {
    execute!(std::io::stdout(), Print(format!("{s}\n")))
        .with_context(|| format!("Cannot print msg `{}` to console", s))
        .unwrap();
}
