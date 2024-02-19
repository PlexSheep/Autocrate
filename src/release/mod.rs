use crate::{config::Config, error::*, serverapi::ApiCollection};

pub async fn release(_cfg: &Config, _apis: &mut ApiCollection) -> Result<()> {
    // TODO: git tag
    // TODO: push to each server

    // TODO: release to each server
    tag(cfg).await?;
    for api in apis.iter_mut() {
        api.push_release().await?;
    }

    for api in apis.iter_mut() {
        api.push_release().await?;
    }

    // TODO: check that the release is made
    // TODO: generate artifacts
    // TODO: upload artifacts
    // TODO: upload artifact signatures

    Ok(())
}

async fn tag(cfg: &Config) -> Result<()> {
    todo!()
}
