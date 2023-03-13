#![allow(warnings)]
use crate::page_builder::PageBuilder;

impl PageBuilder {
    pub fn output(&self) -> String {
        let mut output: Vec<String> = vec![];
        output.push(
            "<!DOCTYPE html><html><body>".to_string(),
        );
        for block in self.blocks() {
            if &block.0 == "title" {
                output.push(self.title_from(&block.1));
            }
            else if &block.0 == "c" {
                output
                    .extend(self.content_from(&block.1));
            }
            else {
                dbg!(block);
            }
        }

        output.push("</body></html>".to_string());
        output.join("\n")
    }
}
