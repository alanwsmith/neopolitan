use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub block_category_kinds: BlockTypes,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockTypes {
    pub raw: Vec<String>,
}

impl Config {
    pub fn default() -> Config {
        let block_category_kinds = BlockTypes {
            raw: vec!["pre".to_string()],
        };
        Config {
            block_category_kinds,
        }
    }
}
