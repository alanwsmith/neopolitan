use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub block_category_kinds: BlockTypes,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockTypes {
    pub csv: Vec<String>,
    pub json: Vec<String>,
    pub raw: Vec<String>,
}

impl Default for Config {
    fn default() -> Config {
        let block_category_kinds = BlockTypes {
            csv: make_vec_of_strings("csv"),
            json: make_vec_of_strings("json|metadata"),
            raw: make_vec_of_strings(
                "cli|code|css|html|javascript|output|pre|raw",
            ),
        };
        Config {
            block_category_kinds,
        }
    }
}

fn make_vec_of_strings(input: &str) -> Vec<String> {
    input.split("|").map(|i| i.to_string()).collect()
}
