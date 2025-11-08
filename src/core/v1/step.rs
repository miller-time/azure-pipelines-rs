//! Steps are a linear sequence of operations that make up a job
//!
//! <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/steps?view=azure-pipelines>

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

/// Steps are a linear sequence of operations that make up a job
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum Step {
    /// Configure how the pipeline checks out source code
    Checkout(CheckoutStep),
    /// Runs a task
    Step(TaskStep),
    /// Define a set of steps in one file and use it multiple times in another
    /// file
    Template(TemplateStep),
}

/// Use `checkout` to configure how the pipeline checks out source code
///
/// <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/steps-checkout?view=azure-pipelines>
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct CheckoutStep {
    /// Configures checkout for the specified repository
    pub checkout: Option<String>,

    /// Set to 'true' to leave the OAuth token in the Git config after the
    /// initial fetch. The default is not to leave it.
    #[serde(default)]
    pub persist_credentials: bool,
}

/// A `task` step runs a task
///
/// <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/steps-task?view=azure-pipelines>
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct TaskStep {
    /// Evaluate this condition expression to determine whether to run this task
    pub condition: Option<String>,

    /// Human-readable name for the task
    pub display_name: Option<String>,

    /// Variables to map into the process's environment
    #[serde(default)]
    pub env: HashMap<String, String>,

    /// Inputs for the task
    pub inputs: HashMap<String, String>,

    /// ID of the step
    pub name: Option<String>,

    /// Number of retries if the task fails
    pub retry_count_on_task_failure: Option<i32>,

    /// Environment in which to run this task
    pub target: Option<StepTarget>,

    /// Name of the task to run
    pub task: Option<String>,
}

/// Tasks run in an execution context, which is either the agent host or a
/// container
///
/// <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/target?view=azure-pipelines>
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct StepTarget {
    /// Container to target (or 'host' for host machine)
    pub container: String,
}

/// Define a set of steps in one file and use it multiple times in another file
///
/// <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/steps-template?view=azure-pipelines>
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct TemplateStep {
    /// Reference to a template for this step
    pub template: Option<String>,

    /// Parameters used in a step template
    #[serde(default)]
    pub parameters: HashMap<String, Value>,
}
