use std::fmt::Display;

use crate::{
    config::{cli::Commands, Config},
    error::*,
};

/// Represents a changelog that is currently under construction.
#[derive(Clone, Debug)]
pub struct Changelog {
    message: Option<String>,
    git_log: Option<String>,
}

impl Changelog {
    pub fn build(cfg: &Config) -> Result<Self> {
        if !cfg.yaml.changelog.enable {
            return Err(ConfigError::IsDisabledButUsed("changelog").into());
        }
        let message: Option<String> = match cfg.cli.command.clone() {
            Commands::Changelog { message } => match message {
                Some(msgs) => Some(msgs.concat()),
                None => None,
            },
        };
        let git_log = Self::make_git_log(cfg)?;
        Ok(Changelog { message, git_log })
    }

    fn make_git_log(cfg: &Config) -> Result<Option<String>> {
        if !cfg.yaml.changelog.enable {
            return Ok(None);
        }
        Ok(Some(format!("todo")))
    }
}

impl Display for Changelog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut full: String = String::new();
        full += "Changelog";
        if self.message.is_some() {
            full += format!("\n\n{}", self.message.clone().unwrap()).as_str();
        }
        if self.git_log.is_some() {
            full += format!("\n\n{}", self.git_log.clone().unwrap()).as_str();
        }
        write!(f, "{full}")
    }
}
