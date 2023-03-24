use crate::section::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Wrapper {
    Post {
        attributes: Option<HashMap<String, String>>,
        children: Option<Vec<Section>>,
    },
}
