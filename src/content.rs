use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Content {
    Text {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        value: Option<String>,
    },
}
