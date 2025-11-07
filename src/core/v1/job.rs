use serde::{Deserialize, Serialize};

use crate::core::v1::step::Step;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Job {
    #[serde(rename = "job")]
    name: Option<String>,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    steps: Vec<Step>,
}
