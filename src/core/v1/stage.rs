use serde::{Deserialize, Serialize};

use crate::core::v1::job::Job;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Stage {
    #[serde(rename = "stage")]
    name: Option<String>,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    jobs: Vec<Job>,
}
