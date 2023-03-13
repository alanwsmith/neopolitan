use crate::section::Section;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Page {
    pub attributes: HashMap<String, String>,
    pub children: Vec<Section>,
    pub categories: Vec<String>,
}
