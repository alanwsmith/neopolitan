use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Chunk {
    H1 {
        attributes: HashMap<String, String>,
        children: Option<Vec<Chunk>>,
    },
    Text {
        attributes: HashMap<String, String>,
        value: Option<String>,
    },
    P {
        attributes: HashMap<String, String>,
        children: Option<Vec<Chunk>>,
    },
    ListItem {
        attributes: HashMap<String, String>,
        children: Option<Vec<Chunk>>,
    },
    Strong {
        attributes: HashMap<String, String>,
        value: Option<String>,
    },
    Link {
        attributes: HashMap<String, String>,
        value: Option<String>,
        url: Option<String>,
    },
    InlineCode {
        attributes: HashMap<String, String>,
        language: Option<String>,
        value: Option<String>,
    },
}
