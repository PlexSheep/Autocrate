use std::{collections::HashMap, fmt::Debug, fs::File, io::BufReader, path::PathBuf};

use git2;
use libpt::log::{debug, error, trace};
use serde::Deserialize;
use url::Url;

use crate::error::*;

pub mod cli;
use cli::Cli;

pub trait YamlConfigSection: Debug + Clone + for<'a> Deserialize<'a> {
    fn check(&self) -> Result<()>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct Changelog {
    pub enable: bool,
    #[serde(alias = "git-log")]
    pub git_log: bool,
}
impl YamlConfigSection for Changelog {
    fn check(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UseCargo {
    publish: bool,
    registries: Vec<String>,
}
impl YamlConfigSection for UseCargo {
    fn check(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Uses {
    cargo: UseCargo,
}
impl YamlConfigSection for Uses {
    fn check(&self) -> Result<()> {
        self.cargo.check()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiAuth {
    pub user: String,
    pub pass: Option<String>,
    pub pass_file: Option<PathBuf>,
}
impl YamlConfigSection for ApiAuth {
    fn check(&self) -> Result<()> {
        if self.pass.is_some() && self.pass_file.is_some() {
            let err = ConfigError::YamlApiAuthBothPass(self.clone()).into();
            error!("{err}");
            return Err(err);
        }
        if self.pass_file.is_some() {
            let file = self.pass_file.clone().unwrap();
            if !file.exists() {
                return Err(ConfigError::PassFileDoesNotExist(file).into());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Api {
    pub r#type: ApiType,
    pub endpoint: Url,
    /// May be left empty if the Api does not need auth or the auth is part of the
    /// [endpoint](Api::endpoint) [Url].
    pub auth: Option<ApiAuth>,
}
impl YamlConfigSection for Api {
    fn check(&self) -> Result<()> {
        self.r#type.check()?;
        match self.endpoint.socket_addrs(|| None) {
            Ok(_) => (),
            Err(err) => return Err(err.into()),
        }
        if self.auth.is_some() {
            self.auth.clone().unwrap().check()?;
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
impl YamlConfigSection for ApiType {
    fn check(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct YamlConfig {
    pub changelog: Changelog,
    pub uses: Uses,
    pub api: HashMap<String, Api>,
}
impl YamlConfigSection for YamlConfig {
    fn check(&self) -> Result<()> {
        self.changelog.check()?;
        self.uses.check()?;
        for api in self.api.values() {
            api.check()?;
        }
        Ok(())
    }
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
    pub cli: Cli,
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
            Err(_err) => {
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

        Ok(Config {
            yaml,
            repo,
            path,
            cli,
        })
    }
}
