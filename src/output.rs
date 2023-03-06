#![allow(warnings)]
use crate::page_builder::PageBuilder;

impl PageBuilder {
    pub fn output(&self) -> String {
        let mut output: Vec<String> = vec![];
        for block in self.blocks() {
            if &block.0 == "title" {
                output.push(format!(
                    "<h1>{}</h1>",
                    &block.1
                ));
            }
        }
        output.join("\n")
        // "asdf".to_string()
    }
}
