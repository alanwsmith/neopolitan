use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BlockType {
    #[serde(rename = "close-block")]
    CloseBlock,
    #[serde(rename = "full-block")]
    FullBlock,
    #[serde(rename = "open-block")]
    OpenBlock,
}
