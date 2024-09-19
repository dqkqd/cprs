use anyhow::Result;
use cprs_cli::{Cmd, DaemonizeClipboard};

#[tokio::main]
async fn main() -> Result<()> {
    if DaemonizeClipboard::copy_from_spawned_process().is_ok() {
        return Ok(());
    }
    Cmd::entry_point().await
}
