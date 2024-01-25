use anyhow;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("no \".autocrate.yaml\" or \".autocrate.yml\" found in repository root")]
    NoYamlFile,
    #[error("the autocrate config file is not a regular file (is it a directory?)")]
    YamlFileIsNotFile,
    #[error("could not find git repository")]
    GitRepoNotFound,
    /// Bad IO operation
    #[error("Bad IO operation")]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("Yaml error")]
    SerdeYaml(#[from] serde_yaml::Error)
}
