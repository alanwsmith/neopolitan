use crate::content::Content;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Block {
    H1 {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        children: Option<Vec<Content>>,
    },
}
