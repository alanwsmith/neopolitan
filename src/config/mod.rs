use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub block_types: BlockTypes,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockTypes {
    pub raw: Vec<String>,
}

impl Config {
    pub fn default() -> Config {
        let mut block_types = BlockTypes {
            raw: vec!["pre".to_string()],
        };
        Config { block_types }
    }
}
