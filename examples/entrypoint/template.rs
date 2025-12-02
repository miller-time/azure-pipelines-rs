use std::{collections::HashMap, error::Error};

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use azure_pipelines_rs::{core::v1::stage::Stage, templates::parameterized::Parameterized};

/// Define an Entrypoint type that has parameters and implement the
/// `Parameterized` trait
#[derive(Default)]
pub struct ExampleEntrypoint {}

impl Parameterized for ExampleEntrypoint {
    type Parameters = ExampleEntrypointParameters;

    fn get_parameters(
        hash_map: &HashMap<String, Value>,
    ) -> Result<Self::Parameters, Box<dyn Error>> {
        let s = serde_yaml::to_string(hash_map)?;
        let parameters: ExampleEntrypointParameters = serde_yaml::from_str(&s)?;
        Ok(parameters)
    }
}

/// Define a type for the Entrypoint Parameters
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ExampleEntrypointParameters {
    pub custom_build_tags: Vec<String>,
    pub containers: HashMap<String, String>,
    pub feature_flags: HashMap<String, Value>,
    pub stages: Vec<Stage>,
}
