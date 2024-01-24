use std::{
    fmt::{write, Debug}, fs::File, path::PathBuf
};

use anyhow::{anyhow, Result};
use git2;
use libpt::log::error;
use serde::Deserialize;

use self::cli::Cli;

pub mod cli;

#[derive(Debug, Clone, Deserialize)]
pub struct Changelog {
    enabled: bool,
    git_log: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Uses {
    cargo: Option<Cargo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Cargo {
    publish: bool,
    registries: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Api {
    api_type: ApiType,
    publish: bool,
    registries: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum ApiType {
    Gitea,
    Gitlab,
    Github,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YamlConfig {
    pub changelog: Changelog,
    pub uses: Uses,
    pub apis: Vec<Api>,
}

pub struct Config {
    pub yaml: YamlConfig,
    pub repo: git2::Repository,
    pub path: PathBuf,
}

impl Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "Config {{yaml: {:?}, repo_path: {:?}}}",
                self.yaml, self.path
            )
        )
    }
}

impl Config {
    pub fn load(cli: Cli) -> Result<Self> {
        let repo = match git2::Repository::open_from_env() {
            Ok(repo) => repo,
            Err(err) => {
                let msg = format!("could not find a git repository: {err:?}");
                error!("{}", msg);
                return Err(anyhow!(msg));
            }
        };
        let mut path = repo.path().to_path_buf();
        path.pop(); // we want the real root, not the `.git` dir

        todo!()
    }
}
