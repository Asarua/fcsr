use colored::Colorize;
use fcsr_pkg::{
  packages::{Package, Packages},
  PkgJson,
};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DependentsGraphOption {
  bump_versions_with_workspace_protocol_only: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct DependencyGraphItem {
  pkg: Package,
  dependencies: Vec<String>,
}

impl DependencyGraphItem {
  fn new(pkg: Package, dependencies: Vec<String>) -> Self {
    Self { pkg, dependencies }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyGraph {
  graph: HashMap<String, DependencyGraphItem>,
  valid: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependentGraph {
  pkg: Package,
  dependents: Vec<String>,
}

pub fn get_dependents_graph(
  packages: Packages,
  opts: Option<DependentsGraphOption>,
) -> HashMap<String, Vec<String>> {
  let mut graph = HashMap::new();
  let DependencyGraph {
    graph: dependency_graph,
    ..
  } = get_dependency_graph(&packages, opts);

  let mut dependents_lookup = HashMap::new();
  packages.packages.iter().for_each(|pkg| {
    dependents_lookup.insert(
      pkg.package_json.name.clone(),
      DependentGraph {
        pkg: pkg.clone(),
        dependents: vec![],
      },
    );
  });

  packages.packages.iter().for_each(|pkg| {
    let dependent = pkg.package_json.name.clone();
    if let Some(val_from_dependency_graph) = dependency_graph.get(&dependent) {
      val_from_dependency_graph
        .dependencies
        .iter()
        .for_each(|dependency| {
          dependents_lookup
            .entry(dependency.clone())
            .and_modify(|dependent_graph| {
              dependent_graph.dependents.push(dependent.clone());
            });
        });
    }
  });

  for (key, value) in dependents_lookup {
    graph.insert(key, value);
  }

  let mut simplified_dependents_graph = HashMap::new();
  for (pkg_info, pkg_name) in graph {
    simplified_dependents_graph.insert(pkg_info, pkg_name.dependents);
  }

  simplified_dependents_graph
}

fn get_dependency_graph(
  packages: &Packages,
  opts: Option<DependentsGraphOption>,
) -> DependencyGraph {
  let mut dependency_graph = DependencyGraph {
    graph: HashMap::new(),
    valid: true,
  };

  let mut packages_by_name: HashMap<String, Package> = HashMap::new();
  packages_by_name.insert(
    packages.root.package_json.name.clone(),
    packages.root.clone(),
  );

  let mut queue = Vec::new();
  queue.push(packages.root.clone());

  for pkg in packages.packages.clone() {
    queue.push(pkg.clone());
    packages_by_name.insert(pkg.package_json.name.clone(), pkg.clone());
  }

  for pkg in queue {
    let name = pkg.package_json.name.clone();
    let dependencies: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]));
    let all_dependencies = get_all_dependencies(pkg.package_json.clone());

    for (dep_name, dep_range) in all_dependencies {
      let rc_dependencies = Rc::clone(&dependencies);
      if let Some(dep_matched) = packages_by_name.get(&dep_name) {
        let use_work_space_range = dep_range.starts_with("workspace:");
        let expected = dep_matched.package_json.version.clone();

        if use_work_space_range {
          if let Ok(workspace_reg) = regex::Regex::new(r#"^workspace:"#) {
            let new_dep_range = workspace_reg.replace(&dep_range, "");
            if matches!(new_dep_range.to_string().as_str(), "*" | "^" | "~") {
              rc_dependencies.borrow_mut().push(dep_name);
              continue;
            }
          }
        } else if let Some(DependentsGraphOption {
          bump_versions_with_workspace_protocol_only,
          ..
        }) = opts
        {
          if bump_versions_with_workspace_protocol_only.is_some()
            && bump_versions_with_workspace_protocol_only.unwrap()
          {
            continue;
          }
        }

        let mut effect_protocol_range = || {
          dependency_graph.valid = false;
          println!(
            r#"Package "{}" must depend on the current version of "{}": "{}" vs "{}""#,
            name.cyan(),
            dep_name.cyan(),
            expected.green(),
            dep_range.red()
          );
        };

        let version_range = get_valid_range(&dep_range);
        let expected_version = semver::Version::parse(&expected);

        if (version_range.is_ok()
          && expected_version.is_ok()
          && !version_range
            .clone()
            .unwrap()
            .matches(&expected_version.unwrap()))
          || is_protocol_range(&dep_range)
        {
          effect_protocol_range();
          continue;
        }

        if version_range.is_err() {
          continue;
        }

        dependencies.borrow_mut().push(dep_name);
      }
    }

    dependency_graph.graph.insert(
      name.clone(),
      DependencyGraphItem::new(pkg.clone(), dependencies.borrow().to_vec()),
    );
  }

  dependency_graph
}

fn is_protocol_range(range: &str) -> bool {
  range.contains(":")
}

fn get_valid_range(potential_range: &str) -> Result<semver::VersionReq, ()> {
  if is_protocol_range(potential_range) {
    return Err(());
  }

  match semver::VersionReq::parse(potential_range) {
    Ok(version_req) => Ok(version_req),
    Err(_) => Err(()),
  }
}

#[derive(Debug, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
enum DependencyItems {
  Dependencies,
  DevDependencies,
  PeerDependencies,
  OptionalDependencies,
}
const ALL_DEPENDENCIES_ITEM: [DependencyItems; 4] = [
  DependencyItems::Dependencies,
  DependencyItems::DevDependencies,
  DependencyItems::PeerDependencies,
  DependencyItems::OptionalDependencies,
];

fn get_all_dependencies(config: PkgJson) -> HashMap<String, String> {
  let mut all_dependencies = HashMap::new();

  match serde_json::to_value(config.clone()) {
    Ok(config_value) => match config_value {
      serde_json::Value::Object(ref config_value_obj) => {
        for dependency_item in ALL_DEPENDENCIES_ITEM {
          if let Ok(dependency_item_string) = serde_json::to_string(&dependency_item) {
            if let Some(dependency_item_value) =
              config_value_obj.get(&dependency_item_string.replace("\"", ""))
            {
              match dependency_item_value {
                serde_json::Value::Object(dependency_item_obj) => {
                  for dependency_item_key in dependency_item_obj.keys() {
                    match dependency_item_obj.get(dependency_item_key).unwrap() {
                      serde_json::Value::String(dep_range) => {
                        if (dep_range.starts_with("link:") || dep_range.starts_with("file:"))
                          && dependency_item == DependencyItems::DevDependencies
                        {
                          continue;
                        }

                        all_dependencies.insert(dependency_item_key.clone(), dep_range.clone());
                      }
                      _ => {}
                    }
                  }
                }
                _ => continue,
              }
            }
          }
        }
      }
      _ => {}
    },
    Err(error) => {
      println!("error: {error:?}")
    }
  }

  all_dependencies
}

#[cfg(test)]
mod test {
  use std::path::PathBuf;

  use super::*;

  macro_rules! gen_hash_map {
    (($($k:expr, $v:expr$(,)?)*)) => {
      {
        let mut hash_map = HashMap::new();
        $(hash_map.insert($k, $v);)*
        hash_map
      }
    };
  }

  #[test]
  fn test_get_all_dependencies() {
    let pkg_json = PkgJson {
      name: String::from("test"),
      version: String::from("1.1.1"),
      dependencies: Some(gen_hash_map!(("A".to_string(), "a".to_string()))),
      dev_dependencies: Some(gen_hash_map!(("B".to_string(), "b".to_string()))),
      peer_dependencies: Some(gen_hash_map!(("C".to_string(), "c".to_string()))),
      optional_dependencies: Some(gen_hash_map!(("D".to_string(), "d".to_string()))),
      resolutions: Some(gen_hash_map!(())),
      private: None,
      publish_config: None,
      workspaces: None,
    };

    println!("{:?}", get_all_dependencies(pkg_json))
  }

  #[test]
  fn should_skip_dependencies_specified_through_the_link_protocol() {
    let pkg: Packages = Packages {
      root: Package {
        package_json: PkgJson::new("root".to_string(), "1.0.0".to_string()),
        dir: PathBuf::from("."),
      },
      tool: fcsr_pkg::packages::Tool::Pnpm,
      packages: vec![
        Package {
          dir: PathBuf::from("foo"),
          package_json: PkgJson {
            name: String::from("foo"),
            version: String::from("1.0.0"),
            dependencies: None,
            peer_dependencies: None,
            dev_dependencies: Some(gen_hash_map!((
              "bar".to_string(),
              "link:../bar".to_string()
            ))),
            optional_dependencies: None,
            resolutions: None,
            private: None,
            publish_config: None,
            workspaces: None,
          },
        },
        Package {
          dir: PathBuf::from("bar"),
          package_json: PkgJson::new("bar".to_string(), "1.0.0".to_string()),
        },
      ],
    };
    let DependencyGraph { valid, graph } = get_dependency_graph(&pkg, None);

    assert!(valid);
    assert_eq!(
      serde_json::to_string(&graph.get("foo").unwrap().dependencies).unwrap(),
      "[]"
    )
  }

  #[test]
  fn should_skip_dependencies_specified_using_a_tag() {
    let pkg = Packages {
      tool: fcsr_pkg::packages::Tool::Pnpm,
      root: Package {
        package_json: PkgJson::new("root".to_string(), "1.0.0".to_string()),
        dir: PathBuf::from("."),
      },
      packages: vec![
        Package {
          dir: PathBuf::from("examples/foo"),
          package_json: PkgJson {
            name: String::from("foo-example"),
            version: String::from("1.0.0"),
            dependencies: Some(gen_hash_map!(("bar".to_string(), "latest".to_string()))),
            peer_dependencies: None,
            dev_dependencies: None,
            optional_dependencies: None,
            resolutions: None,
            private: None,
            publish_config: None,
            workspaces: None,
          },
        },
        Package {
          dir: PathBuf::from("packages/bar"),
          package_json: PkgJson::new("bar".to_string(), "1.0.0".to_string()),
        },
      ],
    };
    let DependencyGraph { valid, graph } = get_dependency_graph(&pkg, None);

    assert!(valid);
    assert_eq!(
      serde_json::to_string(&graph.get("foo-example").unwrap().dependencies).unwrap(),
      "[]"
    )
  }

  #[test]
  fn should_set_valid_to_false_if_the_link_protocol_is_used_in_a_non_dev_dep() {
    let pkg: Packages = Packages {
      root: Package {
        package_json: PkgJson::new("root".to_string(), "1.0.0".to_string()),
        dir: PathBuf::from("."),
      },
      tool: fcsr_pkg::packages::Tool::Pnpm,
      packages: vec![
        Package {
          dir: PathBuf::from("foo"),
          package_json: PkgJson {
            name: String::from("foo"),
            version: String::from("1.0.0"),
            dependencies: Some(gen_hash_map!((
              "bar".to_string(),
              "link:../bar".to_string()
            ))),
            peer_dependencies: None,
            dev_dependencies: None,
            optional_dependencies: None,
            resolutions: None,
            private: None,
            publish_config: None,
            workspaces: None,
          },
        },
        Package {
          dir: PathBuf::from("bar"),
          package_json: PkgJson::new("bar".to_string(), "1.0.0".to_string()),
        },
      ],
    };
    let DependencyGraph { valid, .. } = get_dependency_graph(&pkg, None);

    assert!(!valid);
  }
}
