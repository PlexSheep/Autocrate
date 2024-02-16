use autocrate::{
    changelog::*,
    config::{
        cli::{Cli, Commands},
        Config,
    },
    release::release,
    serverapi::init_servers,
    publish::publish,
    error::*,
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
            init_servers(&cfg).await?;
            release(&cfg).await?;
        }
        Commands::Publish { .. } => {
            publish(&cfg).await?;
        }
    };
    Ok(())
}
