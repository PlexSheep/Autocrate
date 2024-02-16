use crate::{config::Config, error::*, serverapi::ApiCollection};

pub async fn release(cfg: &Config, apis: ApiCollection) -> Result<()> {
    // TODO: git tag
    // TODO: version bump
    // TODO: version select interactive
    // TODO: version select automated
    // TODO: push to each server
    // TODO: release to each server
    todo!()
}
