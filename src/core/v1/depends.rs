//! The `dependsOn` property accepts `string` or `[ string ]`

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum DependsOn {
    Single(String),
    Multi(Vec<String>),
}
