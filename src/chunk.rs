use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Chunk {
    H1 {
        // TODO: Change attributes to Vec
        attributes: Option<HashMap<String, String>>,
        children: Option<Vec<Chunk>>,
    },
    InlineCode {
        // TODO: Change attributes to Vec
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        language: Option<String>,
        value: Option<String>,
    },
    Link {
        // TODO: Change attributes to Vec
        attributes: Option<HashMap<String, String>>,
        url: Option<String>,
        value: Option<String>,
    },
    ListItem {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        children: Option<Vec<Chunk>>,
    },
    P {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        children: Option<Vec<Chunk>>,
    },
    Strong {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        value: Option<String>,
    },
    Text {
        // TODO: Change this to Option
        value: String,
    },
    Placeholder,
}
