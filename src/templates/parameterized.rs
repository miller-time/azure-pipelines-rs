use std::{collections::HashMap, error::Error};

use serde_yaml::Value;

pub trait Parameterized {
    type Parameters;
    fn get_parameters(
        hash_map: &HashMap<String, Value>,
    ) -> Result<Self::Parameters, Box<dyn Error>>;
}
