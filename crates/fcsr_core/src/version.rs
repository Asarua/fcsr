use std::path::PathBuf;

use fcsr_metadata::Version;
use thiserror::Error;

pub fn run_version(command: Version, pwd: PathBuf) -> anyhow::Result<(), VersionError> {
  Ok(())
}

#[derive(Debug, Error)]
pub enum VersionError {}
