use crate::content::Content;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Block {
    P {
        attributes: Option<HashMap<String, String>>,
        children: Option<Vec<Content>>,
    },
}
