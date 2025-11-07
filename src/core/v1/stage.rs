use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::core::v1::{depends::DependsOn, job::Job};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Stage {
    #[serde(rename = "stage")]
    pub name: Option<String>,

    #[serde(rename = "displayName")]
    display_name: Option<String>,

    #[serde(rename = "dependsOn")]
    pub depends_on: Option<DependsOn>,

    condition: Option<String>,

    #[serde(default)]
    variables: HashMap<String, Value>,

    pub jobs: Vec<Job>,

    template: Option<String>,

    #[serde(default)]
    parameters: HashMap<String, Value>,
}
