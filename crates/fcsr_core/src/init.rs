use std::path::PathBuf;

use fcsr_metadata::Init;

pub fn run_init(_: Init, pwd: PathBuf) -> anyhow::Result<(), InitError> {
  let base = pwd.join(".changeset");
  let config_base = base.join("config.json");

  if base.exists() {
    if !config_base.exists() {
      if base.join("config.js").exists() {
        return Err(InitError::PreVersion);
      } else {
        std::fs::write(config_base, "123");
      }
    } else {
      return Err(InitError::BaseHasExist);
    }
  }

  Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum InitError {
  #[error("It looks like you already have changesets initialized. You should be able to run changeset commands no problems.")]
  BaseHasExist,
  #[error(r#"
  It looks like you're using the version 1 `.changeset/config.js` file
  The format of the config object has significantly changed in v2 as well
  - we thoroughly recommend looking at the changelog for this package for what has changed
  Changesets will write the defaults for the new config, remember to transfer your options into the new config at `.changeset/config.json`
"#)]
  PreVersion,
  #[error(
    r#"
  It looks like you don't have a config file
  The default config file will be written at `.changeset/config.json`
"#
  )]
  NotHave,
}
