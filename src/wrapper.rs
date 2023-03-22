use crate::section::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
pub enum Wrapper {
    Page { children: Option<Vec<Section>> },
}
