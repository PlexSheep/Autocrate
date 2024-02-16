use autocrate::{
    changelog::*,
    config::{
        cli::{Cli, Commands},
        Config,
    },
    error::*,
    publish::publish,
    release::release,
    serverapi::init_servers,
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
            let mut apis = init_servers(&cfg).await?;
            release(&cfg, &mut apis).await?;
        }
        Commands::Publish { .. } => {
            publish(&cfg).await?;
        }
    };
    Ok(())
}
