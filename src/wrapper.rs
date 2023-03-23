use crate::section::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Wrapper {
    Post {
        attributes: Option<Vec<(String, String)>>,
        children: Option<Vec<Section>>,
    },
}
