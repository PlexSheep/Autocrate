use autocrate::{
    changelog::*,
    config::{
        cli::{Cli, Commands},
        Config,
    },
    error::*,
    publish::publish,
    release::release,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::cli_parse();
    let cfg = Config::load(&cli)?;

    match cli.command {
        Commands::Changelog { .. } => {
            println!("{}", Changelog::build(&cfg)?);
        }
        Commands::Release { .. } => {
            release(&cfg)?;
        }
        Commands::Publish { .. } => {
            publish(&cfg)?;
        }
    };
    Ok(())
}
