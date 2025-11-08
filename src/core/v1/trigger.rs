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
    branches: Option<TriggerItem>,

    /// File paths to include or exclude for triggering a run
    paths: Option<TriggerItem>,
}

/// Lists of items to include or exclude for trigger events
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct TriggerItem {
    /// List of items to include
    #[serde(default)]
    include: Vec<String>,

    /// List of items to exclude
    #[serde(default)]
    exclude: Vec<String>,
}
