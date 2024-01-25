use std::{collections::HashMap, fmt::Debug, fs::File, io::BufReader, path::PathBuf};

use git2;
use libpt::log::{debug, error, trace};
use serde::Deserialize;
use url::Url;

use crate::error::*;

pub mod cli;
use cli::Cli;

#[derive(Debug, Clone, Deserialize)]
pub struct Changelog {
    enable: bool,
    #[serde(alias="git-log")]
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
pub struct ApiAuth {
    user: String,
    pass: Option<String>,
    pass_file: Option<PathBuf>,
}

impl ApiAuth {
    pub fn check(&self) -> Result<()> {
        if self.pass.is_some() && self.pass_file.is_some() {
            let err = ConfigError::YamlApiAuthBothPass(self.clone()).into();
            error!("{err}");
            return Err(err);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Api {
    r#type: ApiType,
    endpoint: Url,
    /// May be left empty if the Api does not need auth or the auth is part of the
    /// [endpoint](Api::endpoint) [Url].
    auth: Option<ApiAuth>,
}
impl Api {
    pub fn check(&self) -> Result<()> {
        if let Some(auth) = &self.auth {
            auth.check()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum ApiType {
    #[serde(alias = "gitea")]
    Gitea,
    #[serde(alias = "gitlab")]
    Gitlab,
    #[serde(alias = "github", alias = "GitHub")]
    Github,
}

#[derive(Debug, Clone, Deserialize)]
pub struct YamlConfig {
    pub changelog: Changelog,
    pub uses: Uses,
    pub api: HashMap<String, Api>,
}

impl YamlConfig {
    /// check if the built configuration is valid
    pub fn check(&self) -> Result<()> {
        for api in &self.api {
            api.1.check()?;
        }
        Ok(())
    }
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
                let err = ConfigError::GitRepoNotFound.into();
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
            let err = ConfigError::NoYamlFile.into();
            error!("{err}");
            return Err(err);
        };
        let yaml_file_path = path.join(yaml_file_name);
        // we can be sure it exists from the checks above
        assert!(yaml_file_path.exists());
        if !yaml_file_path.is_file() {
            let err = ConfigError::YamlFileIsNotFile.into();
            error!("{err}");
            return Err(err);
        }

        let yaml_rd = BufReader::new(File::open(yaml_file_path)?);
        debug!("reading yaml config and building data structure");
        let yaml: YamlConfig = serde_yaml::from_reader(yaml_rd)?;
        trace!("load config:\n{:#?}", yaml);
        yaml.check()?;
        debug!("built and checked yaml config");

        Ok(Config { yaml, repo, path })
    }
}
