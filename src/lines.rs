#![allow(warnings)]
use crate::page_builder::PageBuilder;

impl PageBuilder {
    pub fn lines(
        &self, source: String,
    ) -> Vec<String> {
        let output: Vec<String> =
            vec![
        "<h1>Welcome To Neopolitan</h1>"
            .to_string(),
        "<p>This is the first test.</p>"
            .to_string(),
        "<p>With a couple of paragraphs.</p>"
            .to_string(),
        "<h3>And another header</h3>".to_string(),
        "<p>And some more content</p>".to_string(),
        "<p>The ends here</p>".to_string(),
    ];
        output
    }
}
