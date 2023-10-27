use super::PackageGroup;
use fcsr_pkg::access_type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ConfigContainer {
  Bool(bool),
  Tuple(String, String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrivatePackages {
  version: bool,
  tag: bool,
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
  use_calculated_version: bool,
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
  only_update_peer_dependents_when_out_of_range: bool,
  update_internal_dependents: UpdateInternalDependents,
  use_calculated_version_for_snapshots: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WrittenConfig {
  changelog: ConfigContainer,
  commit: ConfigContainer,
  fixed: Vec<PackageGroup>,
  linked: Vec<PackageGroup>,
  access: access_type::AccessType,
  base_branch: String,
  changed_file_patterns: PackageGroup,
  private_packages: PrivatePackages,
  update_internal_dependencies: UpdateInternalDependencies,
  ignore: Vec<String>,
  bump_version_with_workspace_protocol_only: Option<bool>,
  #[serde(rename(serialize = "___experimentalUnsafeOptions_WILL_CHANGE_IN_PATCH"))]
  experimental_unsafe_options_will_change_in_path: ExperimentalOptions,
  snapshot: Snapshot,
}
