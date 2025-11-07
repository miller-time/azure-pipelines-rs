use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Step {
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    inputs: Option<StepInputs>,
    target: Option<StepTarget>,
    task: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct StepInputs {
    script: Option<String>,
    #[serde(rename = "targetType")]
    target_type: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct StepTarget {
    container: String,
}
