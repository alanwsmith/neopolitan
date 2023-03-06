use crate::page_builder::PageBuilder;

impl PageBuilder {
    pub fn content(
        &self, source: String,
    ) -> Vec<String> {
        let mut lines: Vec<String> = vec![];
        lines.push(format!("<p>{}</p>", source,));

        lines
    }
}
