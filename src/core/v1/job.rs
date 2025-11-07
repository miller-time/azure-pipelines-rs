use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::core::v1::{depends::DependsOn, step::Step};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Job {
    #[serde(rename = "job")]
    pub name: Option<String>,

    #[serde(rename = "displayName")]
    display_name: Option<String>,

    #[serde(rename = "dependsOn")]
    pub depends_on: Option<DependsOn>,

    condition: Option<String>,

    pool: Option<String>,

    #[serde(default)]
    variables: HashMap<String, Value>,

    #[serde(default)]
    steps: Vec<Step>,

    template: Option<String>,

    #[serde(default)]
    pub parameters: HashMap<String, Value>,
}
