use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
}
