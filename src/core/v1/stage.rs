//! Stages
//!
//! <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/stages?view=azure-pipelines>

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::core::v1::{depends::DependsOn, job::Job};

/// Stages
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum Stage {
    /// Stages are a collection of related jobs
    Stage(StageWithJobs),
    /// You can define a set of stages in one file and use it multiple times in other files
    Template(StageWithTemplate),
}

/// Stages are a collection of related jobs. By default, stages run
/// sequentially. Each stage starts only after the preceding stage is complete
/// unless otherwise specified via the `dependsOn` property.
///
/// <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/stages-stage?view=azure-pipelines>
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct StageWithJobs {
    /// ID of the stage
    #[serde(rename = "stage")]
    pub name: Option<String>,

    /// Human-readable name for the stage
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Any stages which must complete before this one. By default stages are
    /// run sequentially in the order defined in the pipeline. Specify
    /// `dependsOn: []` for a stage if it shouldn't depend on the previous stage
    /// in the pipeline.
    #[serde(rename = "dependsOn")]
    pub depends_on: Option<DependsOn>,

    /// Evaluate this condition expression to determine whether to run this stage
    pub condition: Option<String>,

    /// Pool where jobs in this stage will run unless otherwise specified
    pub pool: Option<String>,

    /// Stage-specific variables
    #[serde(default)]
    pub variables: HashMap<String, Value>,

    /// Jobs which make up the stage
    pub jobs: Vec<Job>,
}

/// You can define a set of stages in one file and use it multiple times in
/// other files.
///
/// <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/stages-template?view=azure-pipelines>
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct StageWithTemplate {
    /// Reference to a template for this stage
    pub template: Option<String>,

    /// Parameters used in a stage template
    #[serde(default)]
    parameters: HashMap<String, Value>,
}
