use crate::page_builder::PageBuilder;

impl PageBuilder {
    pub fn content(
        &self, source: String,
    ) -> Vec<String> {
        return self.content_dev(source);

        // let mut lines: Vec<String> = vec![];
        // lines.push(format!("<p>{}</p>", source,));
        // lines
    }
}
