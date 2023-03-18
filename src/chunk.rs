use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Chunk {
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
    ListItem {
        attributes: Option<HashMap<String, String>>,
        children: Option<Vec<Chunk>>,
    },
    P {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        children: Option<Vec<Chunk>>,
    },
    Text {
        value: String,
    },
    Placeholder,
}
