use crate::section::Section;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize)]
pub struct Page {
    pub attributes: Option<HashMap<String, String>>,
    pub children: Vec<Section>,
}
