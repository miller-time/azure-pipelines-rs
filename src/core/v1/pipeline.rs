//! Pipeline that extends a template
//!
//! <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/pipeline?view=azure-pipelines#pipelineextends>

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::core::v1::{
    extends::Extends,
    trigger::{ResourceTrigger, Trigger},
};

/// Pipeline that extends a template
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Pipeline {
    /// Extends a template
    pub extends: Extends,

    /// Pipeline run number
    pub name: Option<String>,

    /// The runtime parameters for this pipeline.
    #[serde(default)]
    pub parameters: Vec<PipelineParameter>,

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
    pub containers: Vec<ContainerResource>,

    /// List of pipeline resources
    #[serde(default)]
    pub pipelines: Vec<PipelineResource>,

    /// List of repository resources
    #[serde(default)]
    pub repositories: Vec<RepositoryResource>,
}

/// A container resource references a container image
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct ContainerResource {
    /// ID for the container. Acceptable values: `[-_A-Za-z0-9]*`
    #[serde(rename = "container")]
    pub name: String,

    /// ID of the service endpoint connecting to a private container registry
    pub endpoint: Option<String>,

    /// Container image tag
    pub image: String,
}

/// If you have an Azure Pipeline that produces artifacts, your pipeline can
/// consume the artifacts by defining a pipeline resource. In Azure DevOps
/// Server 2020 and higher, you can also enable pipeline completion triggers
/// using a pipeline resource.
///
/// <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/resources-pipelines-pipeline?view=azure-pipelines>
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct PipelineResource {
    /// ID of the pipeline resource
    pub pipeline: String,

    /// Name of the pipeline that produces the artifact
    pub source: String,

    /// Specify `none` to disable, `true` to include all branches, or use the
    /// full syntax
    pub trigger: Option<ResourceTrigger>,
}

/// The `repository` keyword lets you specify an external repository. Use a
/// repository resource to reference an additional repository in your pipeline.
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct RepositoryResource {
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

/// A parameter represents a value passed to a pipeline.
///
/// <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/parameters-parameter?view=azure-pipelines>
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct PipelineParameter {
    /// Parameter name
    pub name: String,

    /// Human-readable name for the parameter
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Parameter type
    #[serde(rename = "type")]
    pub parameter_type: String,

    /// Default value -- if there is no default, then it's required for the user
    /// to specify a value at runtime
    pub default: Option<Value>,

    /// Allowed list of values (for some data types)
    #[serde(default)]
    pub values: Vec<String>,
}
