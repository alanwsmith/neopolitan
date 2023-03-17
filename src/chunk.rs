use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Chunk {
    H1 {
        attributes: HashMap<String, String>,
        children: Vec<Chunk>,
    },
    Text {
        value: String,
    },
}
