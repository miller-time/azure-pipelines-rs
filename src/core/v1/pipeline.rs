//! Pipeline that extends a template
//!
//! <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/pipeline?view=azure-pipelines#pipelineextends>

use serde::{Deserialize, Serialize};

use crate::core::v1::{extends::Extends, trigger::Trigger};

/// Pipeline that extends a template
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Pipeline {
    /// Extends a template
    pub extends: Extends,

    /// Pipeline run number
    pub name: Option<String>,

    /// Pool where jobs in this pipeline will run unless otherwise specified
    pub pool: Option<String>,

    /// Pull request triggers
    pub pr: Option<String>,

    /// Containers and repositories used in the build
    pub resources: Option<PipelineResources>,

    /// Continuous integration triggers
    pub trigger: Option<Trigger>,

    /// Variables for this pipeline
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
