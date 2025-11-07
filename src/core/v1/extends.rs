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
#[serde(rename_all = "camelCase")]
pub struct ExtendsParameters {
    #[serde(default)]
    containers: HashMap<String, String>,

    #[serde(default)]
    custom_build_tags: Vec<String>,

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
#[serde(rename_all = "camelCase")]
pub struct GolangFeatureFlags {
    internal_module_proxy: GoProxy,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct GoProxy {
    enabled: bool,
}
