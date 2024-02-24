use std::{env::VarError, path::PathBuf, process::ExitStatus, string::FromUtf8Error};

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
    #[error("Could not generate the changelog")]
    ChangelogError(#[from] ChangelogError),
    #[error("Server Api error")]
    ServerApiError(#[from] ServerApiError)
}

#[derive(Error, Debug)]
pub enum ServerApiError {
    #[error(transparent)]
    ParseUrl(#[from] url::ParseError),
    #[error(transparent)]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error(transparent)]
    ReqwestErr(#[from] reqwest::Error)
}

#[derive(Error, Debug)]
pub enum ChangelogError {
    #[error("changelog has 'enabled = false' in the yaml config")]
    IsDisabledButUsed,
    #[error("error while using `git log`, is git installed?")]
    GitCommandError,
    #[error("error while using `git log`, could not format stdout with utf8")]
    GitUTF8Error(#[from] FromUtf8Error),
    #[error("git exited with status {0}: {1}")]
    GitBadStatus(ExitStatus, String),
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
    #[error("password provided as file, but does not exist: {0}")]
    PassFileDoesNotExist(PathBuf),
    #[error("config requires environment variable {0}, but {0} is not set")]
    EnvNotSet(String),
    #[error("Bad value for environment variable: {0}")]
    BadEnv(#[from] VarError),
}
