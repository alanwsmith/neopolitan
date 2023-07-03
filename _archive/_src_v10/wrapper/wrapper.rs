use crate::section::section::*;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Wrapper {
    Page { children: Option<Vec<Section>> },
}
