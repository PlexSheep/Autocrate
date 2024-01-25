use anyhow;
use thiserror::Error;

use crate::config::ApiAuth;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    /// Bad IO operation
    #[error("Bad IO operation")]
    IO(#[from] std::io::Error),
    #[error("Bad configuration file")]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("Yaml error")]
    SerdeYaml(#[from] serde_yaml::Error),
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("could not find git repository")]
    GitRepoNotFound,
    #[error("no \".autocrate.yaml\" or \".autocrate.yml\" found in repository root")]
    NoYamlFile,
    #[error("the autocrate config file is not a regular file (is it a directory?)")]
    YamlFileIsNotFile,
    #[error("api {0:?} provides both a `pass` and a `pass_file`")]
    YamlApiAuthBothPass(ApiAuth),
}
