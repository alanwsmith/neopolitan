use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Chunk {
    // TODO: Change the non option stuff to
    // options.
    H1 {
        attributes: HashMap<String, String>,
        children: Vec<Chunk>,
    },
    InlineCode {
        attributes: Option<HashMap<String, String>>,
        language: Option<String>,
        value: Option<String>,
    },
    P {
        attributes: HashMap<String, String>,
        children: Vec<Chunk>,
    },
    Text {
        value: String,
    },
}
