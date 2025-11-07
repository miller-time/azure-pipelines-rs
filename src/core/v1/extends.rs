use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::core::v1::stage::Stage;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Extends {
    template: String,
    parameters: ExtendsParameters,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct ExtendsParameters {
    #[serde(default)]
    containers: HashMap<String, String>,
    #[serde(rename = "customBuildTags", default)]
    custom_build_tags: Vec<String>,
    #[serde(rename = "featureFlags")]
    feature_flags: Option<FeatureFlags>,
    stages: Vec<Stage>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct FeatureFlags {
    golang: GolangFeatureFlags,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct GolangFeatureFlags {
    #[serde(rename = "internalModuleProxy")]
    internal_module_proxy: GoProxy,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct GoProxy {
    enabled: bool,
}
