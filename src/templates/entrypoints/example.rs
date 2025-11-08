use std::{collections::HashMap, error::Error};

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::{core::v1::stage::Stage, templates::parameterized::Parameterized};

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

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct ExampleEntrypointParameters {
    pub stages: Vec<Stage>,
}
