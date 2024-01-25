use anyhow::Result;

mod config;
mod error;
use config::{cli::Cli, Config};

fn main() -> Result<()> {
    let cli = Cli::cli_parse();
    let _config = Config::load(cli.clone())?;

    todo!()
}
