use crate::chunk::Chunk;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Section {
    Title {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        children: Option<Vec<Chunk>>,
    },
}
