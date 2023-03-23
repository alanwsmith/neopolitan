use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Chunk {
    H1 {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        children: Option<Vec<Chunk>>,
    },
    Text {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        value: Option<String>,
    },
    P {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        children: Option<Vec<Chunk>>,
    },
    ListItem {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        children: Option<Vec<Chunk>>,
    },
    Strong {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        value: Option<String>,
    },
    Link {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        value: Option<String>,
        url: Option<String>,
    },
    InlineCode {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        language: Option<String>,
        value: Option<String>,
    },
}
