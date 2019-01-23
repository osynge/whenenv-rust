use std::collections::HashMap;

extern crate serde;
extern crate serde_json;

use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
use std::fmt;
use std::marker::PhantomData;

#[derive(Deserialize, Debug)]
pub struct JdlVariables {
    #[serde(default = "default_vec_string")]
    pub require_keys: Vec<String>,

    #[serde(default = "default_vec_string")]
    pub provides_keys: Vec<String>,

    #[serde(default = "default_hashmap_string_sting")]
    pub require_values: HashMap<String, String>,
}

impl Default for JdlVariables {
    fn default() -> Self {
        JdlVariables {
            require_keys: vec![],
            provides_keys: vec![],
            require_values: HashMap::new(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct DeserializeJdl {
    #[serde(default = "default_string")]
    pub name: String,
    #[serde(default = "default_vec_string")]
    pub depends: Vec<String>,
    #[serde(default = "default_vec_string")]
    pub provides: Vec<String>,
    #[serde(default = "default_string")]
    pub script: String,
    #[serde(default)]
    pub variables: JdlVariables,
}

fn default_string() -> String {
    "".to_string()
}

fn default_vec_string() -> Vec<String> {
    vec![]
}

fn default_hashmap_string_sting() -> HashMap<String, String> {
    HashMap::new()
}
