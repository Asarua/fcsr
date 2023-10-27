use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::PkgJson;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Tool {
  Yarn,
  Bolt,
  Pnpm,
  Lerna,
  Root,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Package {
  pub package_json: PkgJson,
  pub dir: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Packages {
  pub tool: Tool,
  pub packages: Vec<Package>,
  pub root: Package,
}
