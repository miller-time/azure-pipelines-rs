//! A push trigger specifies which branches cause a continuous integration build
//! to run
//!
//! <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/trigger?view=azure-pipelines>
use serde::{Deserialize, Serialize};

/// A push trigger specifies which branches cause a continuous integration build
/// to run
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum Trigger {
    /// Disable CI triggers
    None(String),
    /// Full syntax for complete control
    Trigger(PipelineTrigger),
}

/// Use the full syntax control for full control over the CI trigger
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct PipelineTrigger {
    /// Branch names to include or exclude for triggering a run
    pub branches: Option<TriggerItem>,

    /// File paths to include or exclude for triggering a run
    pub paths: Option<TriggerItem>,
}

/// Lists of items to include or exclude for trigger events
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct TriggerItem {
    /// List of items to include
    #[serde(default)]
    pub include: Vec<String>,

    /// List of items to exclude
    #[serde(default)]
    pub exclude: Vec<String>,
}

/// Specify `none` to disable, `true` to include all branches, or use the full
/// syntax
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum ResourceTrigger {
    Simple(String),
    Full(PipelineResourceTrigger),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct PipelineResourceTrigger {
    /// Branches to include or exclude for triggering a run
    pub branches: Option<TriggerItem>,

    /// List of tags that when matched will trigger the pipeline
    pub tags: Option<TriggerItem>,
}
