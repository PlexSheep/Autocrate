use std::{fmt::Display, process::Command};

use crate::{config::Config, error::*};

/// Represents a changelog that is currently under construction.
#[derive(Clone, Debug)]
pub struct Changelog {
    git_log: Option<String>,
}

impl Changelog {
    pub fn build(cfg: &Config) -> Result<Self> {
        if !cfg.yaml.changelog.enable {
            return Err(ChangelogError::IsDisabledButUsed.into());
        }
        let git_log = Self::make_git_log(cfg)?;
        Ok(Changelog { git_log })
    }

    fn make_git_log(cfg: &Config) -> Result<Option<String>> {
        if !cfg.yaml.changelog.enable {
            return Ok(None);
        }
        let mut cmd = Command::new("git");
        cmd.arg("log").arg("--oneline").arg("--decorate");
        let out = cmd.output()?;
        // FIXME: this does not catch fancy colors
        let buf = String::from_utf8(out.stdout).map_err(|err|{
            ChangelogError::GitUTF8Error(err)
        })?;

        Ok(Some(buf))
    }
}

impl Display for Changelog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut full: String = String::new();
        full += "Changelog";
        if self.git_log.is_some() {
            full += format!("\n\n{}", self.git_log.clone().unwrap()).as_str();
        }
        write!(f, "{full}")
    }
}
