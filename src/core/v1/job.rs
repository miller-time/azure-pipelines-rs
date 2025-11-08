//! Specifies the jobs that make up the work of a stage
//!
//! <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/jobs?view=azure-pipelines>

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::core::v1::{depends::DependsOn, step::Step};

/// Specifies the jobs that make up the work of a stage
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum Job {
    /// A job is a collection of steps run by an agent or on a server
    Job(JobWithSteps),
    /// A set of jobs defined in a template
    Template(JobWithTemplate),
}

/// A job is a collection of steps run by an agent or on a server
///
/// <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/jobs-job?view=azure-pipelines>
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct JobWithSteps {
    /// ID of the job. Acceptable values: Valid names may only contain
    /// alphanumeric characters and '_' and may not start with a number.
    #[serde(rename = "job")]
    pub name: Option<String>,

    /// Human-readable name for the job
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Any jobs which must complete before this one
    #[serde(rename = "dependsOn")]
    pub depends_on: Option<DependsOn>,

    /// Evaluate this condition expression to determine whether to run this job
    pub condition: Option<String>,

    /// Pool where this job will run
    pub pool: Option<String>,

    /// Time to wait for this job to complete before the server kills it
    #[serde(rename = "timeoutInMinutes")]
    pub timeout_in_minutes: Option<String>,

    /// Job-specific variables
    #[serde(default)]
    pub variables: HashMap<String, Value>,

    /// A list of steps to run
    #[serde(default)]
    pub steps: Vec<Step>,
}

/// A set of jobs defined in a template
///
/// <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/jobs-template?view=azure-pipelines>
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct JobWithTemplate {
    /// Reference to a template for this deployment
    pub template: Option<String>,

    /// Parameters used in a deployment template
    #[serde(default)]
    pub parameters: HashMap<String, Value>,
}
