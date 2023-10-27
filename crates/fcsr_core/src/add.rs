use std::path::PathBuf;

use anyhow::anyhow;
use fcsr_metadata::Add;
use thiserror::Error;

pub fn run_add(command: Add, pwd: PathBuf) -> anyhow::Result<(), AddError> {
  Ok(())
}

#[derive(Debug, Error)]
pub enum AddError {
  #[error("this is test {0}")]
  Test(String),
}
