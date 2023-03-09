use crate::page_builder::PageBuilder;

impl PageBuilder {
    pub fn title_from(
        &self, input: &str,
    ) -> String {
        format!("<h1>{}</h1>", input)
    }
}
