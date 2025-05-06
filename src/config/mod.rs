use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub block_category_kinds: BlockTypes,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockTypes {
    pub csv: Vec<String>,
    pub json: Vec<String>,
    pub list: Vec<String>,
    pub raw: Vec<String>,
}

impl Default for Config {
    fn default() -> Config {
        // NOTE: the neo-example- items are built in to make demos
        // easier. Part of the spec is that the ``neo-`` namespace
        // is reserved. I don't expect to do much with it, but
        // things like this are helpful for demos.
        let block_category_kinds = BlockTypes {
            csv: make_vec_of_strings("csv|neo-example-csv-table"),
            json: make_vec_of_strings("json|metadata|neo-example-json-table"),
            list: make_vec_of_strings("list|notes|warnings"),
            raw: make_vec_of_strings(
                "cli|code|css|html|javascript|output|path|pre|raw",
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
