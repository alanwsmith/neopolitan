use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Chunk {
    // TODO: Change the non option stuff to
    // options.
    H1 {
        attributes: Option<HashMap<String, String>>,
        children: Option<Vec<Chunk>>,
    },
    InlineCode {
        attributes: Option<HashMap<String, String>>,
        language: Option<String>,
        value: Option<String>,
    },
    Link {
        attributes: Option<HashMap<String, String>>,
        url: Option<String>,
        value: Option<String>,
    },
    P {
        attributes: Option<HashMap<String, String>>,
        children: Option<Vec<Chunk>>,
    },
    Text {
        value: String,
    },
}
