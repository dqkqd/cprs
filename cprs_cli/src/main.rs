use anyhow::Result;
use cprs_cli::Cmd;

#[tokio::main]
async fn main() -> Result<()> {
    Cmd::entry_point().await
}
