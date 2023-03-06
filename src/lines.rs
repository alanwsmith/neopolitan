use crate::page_builder::PageBuilder;

impl PageBuilder {
    pub fn lines(
        &self, blocks: Vec<(String, String)>,
    ) -> Vec<String> {
        let mut lines: Vec<String> = vec![];
        for block in blocks {
            if block.0 == "title" {
                lines.push(
                    format!("<h1>{}</h1>", block.1)
                        .to_string(),
                );
            }
            if block.0 == "c" {
                lines.push(format!(
                    "<p>{}</p>",
                    block.1
                ))
            }
            if block.0 == "h2" {
                lines.push(
                    format!("<h2>{}</h2>", block.1)
                        .to_string(),
                );
            }
        }
        lines
    }
}
