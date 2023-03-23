use crate::section::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "t", content = "c")]
pub enum Wrapper {
    Page {
        attributes: Option<Vec<(String, String)>>,
        children: Option<Vec<Section>>,
    },
}
