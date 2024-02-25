use std::process::Command;

use git2;

use crate::{config::Config, error::Result};

pub async fn tag(cfg: &Config) -> Result<git2::Tag> {
    // TODO: error handling
    // TODO: allow force
    // TODO: allow setting a message
    // TODO: maybe using git as cmd is fancier?
    let target = cfg
        .repo
        .find_object(
            cfg.repo.head().unwrap().target().unwrap(),
            Some(git2::ObjectType::Commit),
        )
        .unwrap();
    let tagger = cfg.repo.signature().expect("could not get signature");
    let message = String::new();
    let force = true;
    let tag = cfg
        .repo
        .tag(
            // &cfg.yaml.version.get_version(),
            "importantversion",
            &target,
            &tagger,
            &message,
            force,
        )
        .unwrap();
    let tag: git2::Tag = cfg.repo.find_tag(tag).unwrap();
    Ok(tag)
}

pub async fn push(cfg: &Config) -> Result<()> {
    // TODO: error handling
    // TODO: maybe using git as lib is fancier?
    Command::new("git").arg("push").status().unwrap();
    Ok(())
}

pub async fn get_commit_sig(cfg: &Config) -> Result<String> {
    // TODO: error handling
    // TODO: maybe using git as cmd is fancier?
    let target = cfg
        .repo
        .find_commit(cfg.repo.head().unwrap().target().unwrap())
        .unwrap();
    Ok(target.id().to_string())
}
