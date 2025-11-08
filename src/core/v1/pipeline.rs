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
    pub variables: Vec<PipelineVariable>,
}

/// Resources specifies builds, repositories, pipelines, and other resources
/// used by the pipeline
///
/// <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/resources?view=azure-pipelines>
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct PipelineResources {
    /// List of container images
    #[serde(default)]
    pub containers: Vec<PipelineContainer>,

    /// List of repository resources
    #[serde(default)]
    pub repositories: Vec<PipelineRepository>,
}

/// A container resource references a container image
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct PipelineContainer {
    /// ID for the container. Acceptable values: `[-_A-Za-z0-9]*`
    #[serde(rename = "container")]
    pub name: String,

    /// ID of the service endpoint connecting to a private container registry
    pub endpoint: Option<String>,

    /// Container image tag
    pub image: String,
}

/// The `repository` keyword lets you specify an external repository. Use a
/// repository resource to reference an additional repository in your pipeline.
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct PipelineRepository {
    /// Alias for the specified repository. Acceptable values: `[-_A-Za-z0-9]*`
    #[serde(rename = "repository")]
    pub alias: String,

    /// Type of repository: git, github, githubenterprise, and bitbucket
    #[serde(rename = "type")]
    pub repository_type: String,

    /// Repository name. Format depends on `repository_type` ("type")
    pub name: String,

    /// ref name to checkout; defaults to 'refs/heads/main'. The branch checked
    /// out by default whenever the resource trigger fires
    #[serde(rename = "ref")]
    pub repository_ref: String,
}

/// Define variables using name/value pairs
///
/// <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/variables?view=azure-pipelines>
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum PipelineVariable {
    /// Reference variables from a variable group
    Group(VariableGroup),

    /// Define variables using name and full syntax
    Variable(ValueVariable),
}

/// Reference variables from a variable group
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct VariableGroup {
    /// Variable group name
    pub group: String,
}

/// Define variables using name and full syntax
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct ValueVariable {
    /// Variable name
    pub name: String,

    /// Variable value
    pub value: String,
}
