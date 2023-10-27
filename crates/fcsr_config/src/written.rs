use super::PackageGroup;
use fcsr_pkg::access_type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigContainerTuple {
  String(String),
  Value(serde_json::Value),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigContainer {
  Bool(bool),
  Tuple(Vec<ConfigContainerTuple>),
  String(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PrivatePackagesEnum {
  Bool(bool),
  PrivatePackages(PrivatePackages),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrivatePackages {
  version: Option<bool>,
  tag: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UpdateInternalDependencies {
  Patch,
  Minor,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
  use_calculated_version: Option<bool>,
  prerelease_template: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UpdateInternalDependents {
  Always,
  OutOfRange,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExperimentalOptions {
  only_update_peer_dependents_when_out_of_range: Option<bool>,
  update_internal_dependents: Option<UpdateInternalDependents>,
  use_calculated_version_for_snapshots: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WrittenConfig {
  #[serde(rename = "$schema")]
  pub schema: Option<String>,
  pub changelog: Option<ConfigContainer>,
  pub commit: Option<ConfigContainer>,
  pub fixed: Option<Vec<PackageGroup>>,
  pub linked: Option<Vec<PackageGroup>>,
  pub access: Option<access_type::AccessType>,
  pub base_branch: Option<String>,
  pub changed_file_patterns: Option<PackageGroup>,
  pub private_packages: Option<PrivatePackagesEnum>,
  pub update_internal_dependencies: Option<UpdateInternalDependencies>,
  pub ignore: Option<Vec<String>>,
  pub bump_version_with_workspace_protocol_only: Option<bool>,
  pub snapshot: Option<Snapshot>,
  #[serde(rename(serialize = "___experimentalUnsafeOptions_WILL_CHANGE_IN_PATCH"))]
  pub experimental_unsafe_options_will_change_in_path: Option<ExperimentalOptions>,
}

impl Default for WrittenConfig {
  fn default() -> Self {
    Self {
      schema: Some(String::from(
        "https://unpkg.com/@changesets/config@latest/schema.json",
      )),
      changelog: Some(ConfigContainer::String(String::from(
        "@changesets/cli/changelog",
      ))),
      commit: Some(ConfigContainer::Bool(false)),
      fixed: Some(Vec::from(Vec::new())),
      linked: Some(Vec::from(Vec::new())),
      access: Some(access_type::AccessType::Restricted),
      base_branch: Some(String::from("master")),
      changed_file_patterns: None,
      private_packages: None,
      update_internal_dependencies: Some(UpdateInternalDependencies::Patch),
      ignore: Some(Vec::new()),
      bump_version_with_workspace_protocol_only: None,
      snapshot: None,
      experimental_unsafe_options_will_change_in_path: None,
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_written_config_default_serialize() {
    let default_written_config = WrittenConfig::default();
    assert!(matches!(
      serde_json::to_string(&default_written_config),
      Ok(_)
    ))
  }

  #[test]
  fn test_written_config_default_deserialize() {
    let config = serde_json::from_str::<WrittenConfig>(
      r#"{"$schema":"https://unpkg.com/@changesets/config@lasest/schema.json","changelog":"@changesets/cli/changelog","commit":false,"fixed":[],"linked":[],"access":"restricted","baseBranch":"master","changedFilePatterns":null,"privatePackages":null,"updateInternalDependencies":"patch","ignore":[],"bumpVersionWithWorkspaceProtocolOnly":null,"snapshot":null,"___experimentalUnsafeOptions_WILL_CHANGE_IN_PATCH":null}"#,
    );
    assert!(matches!(config, Ok(_)))
  }
}
