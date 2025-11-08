//! Extend a pipeline using a template
//!
//! <https://learn.microsoft.com/en-us/azure/devops/pipelines/yaml-schema/extends?view=azure-pipelines>

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::core::v1::stage::Stage;

/// Extend a pipeline using a template
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Extends {
    /// The template referenced by the pipeline to extend
    pub template: String,

    /// Parameters used in the extend
    pub parameters: ExtendsParameters,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ExtendsParameters {
    /// Containers used in the build
    #[serde(default)]
    pub containers: HashMap<String, String>,

    /// Useless strings that display on the pipeline page
    #[serde(default)]
    pub custom_build_tags: Vec<String>,

    /// Enable optional pipeline features
    pub feature_flags: Option<FeatureFlags>,

    /// Stages are groups of jobs that can run without human intervention
    pub stages: Vec<Stage>,
}

/// Enable optional pipeline features
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct FeatureFlags {
    /// Enable optional pipeline features for Go
    golang: GolangFeatureFlags,
}

/// Enable optional pipeline features for Go
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct GolangFeatureFlags {
    /// Enable Go module proxy
    internal_module_proxy: GoProxy,
}

/// Enable Go module proxy
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct GoProxy {
    /// Enable Go module proxy
    enabled: bool,
}
