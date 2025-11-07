use serde::{Deserialize, Serialize};

use crate::core::v1::{extends::Extends, trigger::Trigger};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Pipeline {
    pub extends: Extends,

    pr: Option<String>,

    resources: Option<PipelineResources>,

    trigger: Option<Trigger>,

    #[serde(default)]
    variables: Vec<PipelineVariable>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct PipelineResources {
    #[serde(default)]
    containers: Vec<PipelineContainer>,
    #[serde(default)]
    repositories: Vec<PipelineRepository>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct PipelineContainer {
    #[serde(rename = "container")]
    name: String,
    endpoint: Option<String>,
    image: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct PipelineRepository {
    #[serde(rename = "repository")]
    alias: String,
    #[serde(rename = "type")]
    repository_type: String,
    name: String,
    #[serde(rename = "ref")]
    repository_ref: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct PipelineVariable {
    group: Option<String>,
    name: Option<String>,
    value: Option<String>,
}
