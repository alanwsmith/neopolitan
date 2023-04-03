use crate::block::Block;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Section {
    Title {
        attributes: Option<HashMap<String, String>>,
        children: Option<Vec<Block>>,
    },
    Placeholder,
}
