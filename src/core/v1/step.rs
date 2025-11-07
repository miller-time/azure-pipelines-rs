use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Step {
    checkout: Option<String>,

    display_name: Option<String>,

    #[serde(default)]
    env: HashMap<String, String>,

    inputs: Option<StepInputs>,

    name: Option<String>,

    #[serde(default)]
    persist_credentials: bool,

    retry_count_on_task_failure: Option<i32>,

    target: Option<StepTarget>,

    task: Option<String>,

    template: Option<String>,

    #[serde(default)]
    parameters: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct StepInputs {
    script: Option<String>,
    target_type: Option<String>,
    working_directory: Option<String>,
    notify_users: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct StepTarget {
    container: String,
}
