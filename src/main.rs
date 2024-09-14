use cprs::config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::load()?;
    cfg.save()?;
    Ok(())
}
