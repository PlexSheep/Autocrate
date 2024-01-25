use std::{
    collections::HashMap, fmt::Debug, fs::File, io::BufReader, path::PathBuf
};

use git2;
use libpt::log::error;
use serde::Deserialize;
use url::Url;

use crate::error::*;

pub mod cli;
use cli::Cli;

#[derive(Debug, Clone, Deserialize)]
pub struct Changelog {
    enable: bool,
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
pub struct ApiAuth { user: String, pass: String }

#[derive(Debug, Clone, Deserialize)]
pub struct Api {
    r#type: ApiType,
    endpoint: Url,
    auth: ApiAuth,
}

#[derive(Debug, Clone, Deserialize)]
pub enum ApiType {
    #[serde(alias="gitea")]
    Gitea,
    #[serde(alias="gitlab")]
    Gitlab,
    #[serde(alias="github", alias="GitHub")]
    Github,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YamlConfig {
    pub changelog: Changelog,
    pub uses: Uses,
    pub api: HashMap<String,Api>,
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
                let err = Error::GitRepoNotFound;
                error!("{err}");
                return Err(err);
            }
        };
        let mut path = repo.path().to_path_buf();
        path.pop(); // we want the real root, not the `.git` dir

        let yaml_file_name = if path.join(".autocrate.yaml").exists() {
            ".autocrate.yaml"
        } else if path.join(".autocrate.yml").exists() {
            ".autocrate.yml"
        } else {
            let err = Error::NoYamlFile;
            error!("{err}");
            return Err(err);
        };
        let yaml_file_path = path.join(yaml_file_name);
        // we can be sure it exists from the checks above
        assert!(yaml_file_path.exists());
        if !yaml_file_path.is_file() {
            let err = Error::YamlFileIsNotFile;
            error!("{err}");
            return Err(err);
        }

        let yaml_rd = BufReader::new(File::open(yaml_file_path)?);
        let yaml: YamlConfig = serde_yaml::from_reader(yaml_rd)?;

        Ok(Config { yaml, repo, path })
    }
}
