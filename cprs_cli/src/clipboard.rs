use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{bail, Context};
use arboard::{Clipboard, SetExtLinux};
use clap::{command, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct DeamonizeClipboardCli {
    key: String,
    file_path: PathBuf,
}

pub struct DaemonizeClipboard;

impl DaemonizeClipboard {
    pub const DAEMONIZE_ARG: &str = "__internal_daemonize";

    pub fn try_spawn_copy_process(file_path: &Path) -> anyhow::Result<()> {
        // https://github.com/1Password/arboard/issues/154#issuecomment-2176736555
        // https://docs.rs/arboard/latest/arboard/trait.SetExtLinux.html#tymethod.wait
        let current_cli = env::current_exe()?;

        let file_path_str = file_path
            .to_str()
            .with_context(|| "Failed to get cleaned file")?
            .to_string();

        if cfg!(target_os = "linux") {
            Command::new(current_cli)
                .args([DaemonizeClipboard::DAEMONIZE_ARG, &file_path_str])
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?;
        } else {
            let mut clipboard =
                Clipboard::new().with_context(|| "Failed to access system clipboard")?;
            let source_content = fs::read_to_string(file_path)?;
            clipboard
                .set_text(source_content)
                .with_context(|| "Failed to copy")?;
        }
        Ok(())
    }

    pub fn copy_from_spawned_process() -> anyhow::Result<()> {
        if cfg!(target_os = "linux") {
            let args = DeamonizeClipboardCli::try_parse()?;
            if args.key == DaemonizeClipboard::DAEMONIZE_ARG {
                let mut clipboard =
                    Clipboard::new().with_context(|| "Failed to access system clipboard")?;
                let source_content = fs::read_to_string(&args.file_path)?;
                clipboard
                    .set()
                    .wait()
                    .text(source_content)
                    .with_context(|| "Cannot copy to clipboard")?;
                return Ok(());
            }
        }
        bail!("Not supported, or no need for copying");
    }
}
