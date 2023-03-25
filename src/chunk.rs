use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Chunk {
    H1 {
        attributes: Option<HashMap<String, Option<String>>>,
        children: Option<Vec<Chunk>>,
    },
    Text {
        attributes: Option<HashMap<String, Option<String>>>,
        value: Option<String>,
    },
    P {
        attributes: Option<HashMap<String, Option<String>>>,
        children: Option<Vec<Chunk>>,
    },
    ListItem {
        attributes: Option<HashMap<String, Option<String>>>,
        children: Option<Vec<Chunk>>,
    },
    Strong {
        attributes: Option<HashMap<String, Option<String>>>,
        value: Option<String>,
    },
    Link {
        attributes: Option<HashMap<String, Option<String>>>,
        value: Option<String>,
        url: Option<String>,
    },
    InlineCode {
        attributes: Option<HashMap<String, Option<String>>>,
        language: Option<String>,
        value: Option<String>,
    },
    Placeholder,
}
