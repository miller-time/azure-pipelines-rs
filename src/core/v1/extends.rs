//! Extend a pipeline using a template
//!
//! <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/extends?view=azure-pipelines>

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

/// Extend a pipeline using a template
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Extends {
    /// The template referenced by the pipeline to extend
    pub template: String,

    /// Parameters used in the extend
    pub parameters: HashMap<String, Value>,
}
