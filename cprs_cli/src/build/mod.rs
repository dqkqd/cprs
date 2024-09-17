use std::{io::Write, path::Path, process};

pub mod bundler;
mod loader;

use anyhow::{Context, Result};

fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}

fn prettify(code: &str) -> Result<String> {
    let mut command = process::Command::new("rustfmt")
        .args(["--config", "newline_style=Unix"])
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn rustfmt process");
    {
        command.stdin.take().unwrap().write_all(code.as_ref())?;
    }
    let out = command.wait_with_output()?;
    let formatted = String::from_utf8(out.stdout).with_context(|| "rustfmt failed")?;

    Ok(formatted)
}
