use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {}

impl Config {
    pub fn default() -> Config {
        Config {}
    }
}
