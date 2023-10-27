use serde::{de::Visitor, Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct PublishConfig {
  #[serde(skip_serializing_if = "Option::is_none", default)]
  access: Option<access_type::AccessType>,
  #[serde(skip_serializing_if = "Option::is_none", default)]
  directory: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none", default)]
  registry: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PkgJson {
  pub name: String,
  pub version: String,
  #[serde(skip_serializing_if = "Option::is_none", default)]
  pub dependencies: Option<HashMap<String, String>>,
  #[serde(skip_serializing_if = "Option::is_none", default)]
  pub peer_dependencies: Option<HashMap<String, String>>,
  #[serde(skip_serializing_if = "Option::is_none", default)]
  pub dev_dependencies: Option<HashMap<String, String>>,
  #[serde(skip_serializing_if = "Option::is_none", default)]
  pub optional_dependencies: Option<HashMap<String, String>>,
  #[serde(skip_serializing_if = "Option::is_none", default)]
  pub resolutions: Option<HashMap<String, String>>,
  #[serde(skip_serializing_if = "Option::is_none", default)]
  pub private: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none", default)]
  pub publish_config: Option<PublishConfig>,
  #[serde(skip_serializing_if = "Option::is_none", default)]
  pub workspaces: Option<Vec<String>>,
}

impl PkgJson {
  pub fn new(name: String, version: String) -> Self {
    Self {
      name,
      version,
      dependencies: None,
      peer_dependencies: None,
      dev_dependencies: None,
      optional_dependencies: None,
      resolutions: None,
      private: None,
      publish_config: None,
      workspaces: None,
    }
  }
}

pub mod access_type {
  use super::*;

  #[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
  #[serde(rename_all = "camelCase")]
  pub enum AccessType {
    Public,
    Restricted,
    Private,
  }

  // impl Serialize for AccessType {
  //   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  //   where
  //     S: serde::Serializer,
  //   {
  //     match self {
  //       Self::Public => serializer.serialize_str("public"),
  //       Self::Restricted => serializer.serialize_str("restricted"),
  //     }
  //   }
  // }

  // #[derive(Debug)]
  // enum StringVisitorError {
  //   ParseFail,
  // }

  // impl std::error::Error for StringVisitorError {}

  // impl serde::ser::Error for StringVisitorError {
  //   fn custom<T>(msg: T) -> Self
  //   where
  //     T: std::fmt::Display,
  //   {
  //     Self::ParseFail
  //   }
  // }

  // impl serde::de::Error for StringVisitorError {
  //   fn custom<T>(msg: T) -> Self
  //   where
  //     T: std::fmt::Display,
  //   {
  //     Self::ParseFail
  //   }
  // }

  // impl Display for StringVisitorError {
  //   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
  //     match self {
  //       Self::ParseFail => f.write_str("from Display: accessType deserialize fail."),
  //     }
  //   }
  // }

  // struct StringVisitor;
  // impl Visitor<'_> for StringVisitor {
  //   type Value = AccessType;
  //   fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
  //   where
  //     E: serde::de::Error,
  //   {
  //     match v.as_str() {
  //       "public" => Ok(AccessType::Public),
  //       "restricted" => Ok(AccessType::Restricted),
  //       _ => Err(E::custom(StringVisitorError::ParseFail)),
  //     }
  //   }

  //   fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
  //     formatter.write_str("from Visitor: accessType deserialize fail.")
  //   }
  // }

  // impl<'de> Deserialize<'de> for AccessType {
  //   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  //   where
  //     D: serde::Deserializer<'de>,
  //   {
  //     deserializer.deserialize_string(StringVisitor)
  //   }
  // }
}

#[cfg(test)]
mod test {
  use serde_json::{json, to_string};

  use super::*;

  #[test]
  fn serialize() {
    let pkg = PkgJson {
      name: String::from("123456"),
      version: String::from("321"),
      dependencies: Some(HashMap::new()),
      peer_dependencies: Some(HashMap::new()),
      dev_dependencies: Some(HashMap::new()),
      optional_dependencies: Some(HashMap::new()),
      resolutions: None,
      private: Some(true),
      publish_config: Some(PublishConfig {
        access: Some(access_type::AccessType::Public),
        directory: Some(String::from("6666")),
        registry: Some(String::from("465")),
      }),
      workspaces: None,
    };
    assert_eq!(
      r#"{"name":"123456","version":"321","dependencies":{},"peerDependencies":{},"devDependencies":{},"optionalDependencies":{},"private":true,"publishConfig":{"access":"public","directory":"6666","registry":"465"}}"#,
      to_string(&pkg).unwrap_or_default()
    )
  }

  #[test]
  fn deserialize() {
    let json = json!({
      "name": "zhangsan",
      "version": "lisi",
      "publishConfig": {
        "access": "public"
      }
    });
    let parsed: PkgJson = serde_json::from_value(json).unwrap();
    assert_eq!(
      parsed,
      PkgJson {
        name: String::from("zhangsan"),
        version: String::from("lisi"),
        dependencies: None,
        peer_dependencies: None,
        dev_dependencies: None,
        optional_dependencies: None,
        resolutions: None,
        private: None,
        workspaces: None,
        publish_config: Some(PublishConfig {
          access: Some(access_type::AccessType::Public),
          directory: None,
          registry: None,
        }),
      }
    );
  }
}
