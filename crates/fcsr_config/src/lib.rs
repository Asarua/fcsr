mod config;
mod dependency_graph;
mod written;

use dependency_graph::get_dependents_graph;
use fcsr_pkg::access_type::AccessType;
use glob::Pattern;
use serde::Serialize;
use std::collections::HashSet;

pub type PackageGroup = Vec<String>;

pub fn parse(json: written::WrittenConfig, packages: fcsr_pkg::packages::Packages) {
  let mut pkg_names: Vec<_> = packages
    .packages
    .iter()
    .map(|package| package.package_json.name.clone())
    .collect();

  let normalized_access = match json.access {
    Some(access) => match access {
      AccessType::Restricted | AccessType::Private => AccessType::Restricted,
      AccessType::Public => AccessType::Public,
    },
    None => AccessType::Restricted,
  };

  let fixed = if let Some(json_fixed) = json.fixed {
    parse_package_group(json_fixed, &mut pkg_names, ParsePackageGroupType::Fixed)
  } else {
    vec![]
  };

  let linked = if let Some(json_linked) = json.linked {
    parse_package_group(json_linked, &mut pkg_names, ParsePackageGroupType::Linked)
  } else {
    vec![]
  };

  let all_fixed_packages: HashSet<String> = HashSet::from_iter(fixed.clone().into_iter().flatten());
  let all_linked_packages: HashSet<String> =
    HashSet::from_iter(linked.clone().into_iter().flatten());

  for fixed_pkg_name in all_fixed_packages {
    if all_linked_packages.contains(&fixed_pkg_name) {
      println!("The package \"{fixed_pkg_name}\" can be found in both fixed and linked groups. A package can only be either fixed or linked.")
    }
  }

  if let Some(json_ignore) = json.ignore {
    let dependents_graph = get_dependents_graph(packages, None);
    for ignored_package in json_ignore.iter() {
      if let Some(dependents) = dependents_graph.get(ignored_package) {
        for dependent in dependents {
          if !json_ignore.contains(dependent) {
            println!(
              r#"The package "{dependent}" depends on the ignored package "{ignored_package}", but "{dependent}" is not being ignored. Please add "{dependent}" to the `ignore` option."#
            );
          }
        }
      }
    }
  }
}

#[derive(Serialize, Debug)]
enum ParsePackageGroupType {
  Fixed,
  Linked,
}

fn parse_package_group(
  group: Vec<PackageGroup>,
  pkg_names: &mut Vec<String>,
  r#type: ParsePackageGroupType,
) -> Vec<PackageGroup> {
  let mut fixed: Vec<Vec<String>> = vec![];

  let mut found_pkg_names = HashSet::<String>::new();
  let mut duplicated_pkg_names = HashSet::<String>::new();

  for fixed_group in group {
    let mut expanded_fixed_group = vec![];
    for fixed_group_item in fixed_group {
      if let Ok(ref pattern) = Pattern::new(&fixed_group_item) {
        pkg_names.iter().for_each(|pkg_name| {
          if pattern.matches(pkg_name) {
            expanded_fixed_group.push(pkg_name.clone());
          }
          // println!(
          //   "The package or glob expression \"{pkg_name}\" specified in the `{}` option does not match any package in the project. You may have misspelled the package name or provided an invalid glob expression. Note that glob expressions must be defined according to https://docs.rs/glob/0.3.1/glob/struct.Pattern.html.",
          //   serde_json::to_string(&r#type).unwrap_or_default()
          // )
        });
      }
    }

    for fixed_pkg_name in expanded_fixed_group.iter() {
      if found_pkg_names.contains(fixed_pkg_name) {
        duplicated_pkg_names.insert(fixed_pkg_name.clone());
      }
      found_pkg_names.insert(fixed_pkg_name.clone());
    }

    fixed.push(expanded_fixed_group);

    if !duplicated_pkg_names.is_empty() {
      for duplicated_pkg_name in duplicated_pkg_names.iter() {
        println!("The package \"{duplicated_pkg_name}\" is defined in multiple sets of fixed packages. Packages can only be defined in a single set of fixed packages. If you are using glob expressions, make sure that they are valid according to https://docs.rs/glob/0.3.1/glob/struct.Pattern.html.")
      }
    }
  }
  fixed
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse() {
    let json = serde_json::json!({
      "changelog": 123456
    });

    match serde_json::from_value::<written::WrittenConfig>(json) {
      Ok(w) => {
        println!("{w:?}")
      }
      Err(e) => {
        println!("{e}")
      }
    }
  }
}
