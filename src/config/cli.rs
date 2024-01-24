use serde;
use serde_yaml;

use std::path::PathBuf;

use libpt::log::{Level, Logger};

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

/// short about section displayed in help
const ABOUT_ROOT: &'static str = r##"
Release Manager for Your Projects on Gitea, GitHub, and GitLab.
"##;
/// longer about section displayed in help, is combined with [the short help](ABOUT_ROOT)
static LONG_ABOUT_ROOT: &'static str = r##""##;

#[derive(Debug, Clone, Parser)]
#[command(
    author,
    version,
    about = ABOUT_ROOT,
    long_about = format!("{}{}", ABOUT_ROOT ,LONG_ABOUT_ROOT),
    help_template =
r#"{about-section}
{usage-heading} {usage}
{all-args}{tab}

autocrate: {version}
Author: {author-with-newline}
"#
    )]
pub struct Cli {
    // clap_verbosity_flag seems to make this a global option implicitly
    /// set a verbosity, multiple allowed (f.e. -vvv)
    #[command(flatten)]
    pub(crate) verbose: Verbosity<InfoLevel>,

    /// show additional logging meta data
    #[arg(long)]
    pub(crate) meta: bool,
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
        return cli;
    }
}
