use std::fmt::Display;

use libpt::log::{Level, Logger};

use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Debug, Clone, Parser)]
#[command(
    author,
    version,
    about,
    long_about,
    help_template = r#"{about-section}
{usage-heading} {usage}
{all-args}{tab}

autocrate: {version}
Author: {author-with-newline}
"#
)]
/// Release Manager for Your Projects on Gitea, GitHub, and GitLab.
pub struct Cli {
    // clap_verbosity_flag seems to make this a global option implicitly
    /// set a verbosity, multiple allowed (f.e. -vvv)
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    /// show additional logging meta data
    #[arg(long)]
    pub meta: bool,

    /// the subcommands are part of this enum
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Changelog {},
    Release {
        // FIXME: allow taking a message like this:
        // `autocrate changelog -m arg1 arg2 arg3`
        // -> msg="arg1 arg2 arg3"
        // Instead of only
        // `autocrate changelog -m "arg1 arg2 arg3"`
        // -> msg="arg1 arg2 arg3"
        //
        // TODO:
        // Perhaps open the $EDITOR of the user if
        // no message is provided, like git does
        //
        // TODO:
        // find a way to make this a global option but only usable with specific subcommands
        #[arg(short, long)]
        message: Option<Vec<String>>,
    },
    Publish {
        // see Commands::Release { message }
        #[arg(short, long)]
        message: Option<Vec<String>>,
    },
}

impl Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Changelog { .. } => "Changelog",
                Self::Release { .. } => "Release",
                Self::Publish { .. } => "Publish",
            }
        )
    }
}

impl Cli {
    pub fn cli_parse() -> Self {
        let cli = Self::parse();
        let ll: Level = match cli.verbose.log_level().unwrap().as_str() {
            "TRACE" => Level::TRACE,
            "DEBUG" => Level::DEBUG,
            "INFO" => Level::INFO,
            "WARN" => Level::WARN,
            "ERROR" => Level::ERROR,
            _ => {
                unreachable!();
            }
        };
        if cli.meta {
            Logger::init(None, Some(ll), true).expect("could not initialize Logger");
        } else {
            // less verbose version
            Logger::init_mini(Some(ll)).expect("could not initialize Logger");
        }
        cli
    }
}
