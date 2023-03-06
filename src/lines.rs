use crate::page_builder::PageBuilder;

impl PageBuilder {
    pub fn lines(
        &self, blocks: Vec<(String, String)>,
    ) -> Vec<String> {
        let mut lines: Vec<String> = vec![];
        for block in blocks.iter() {
            let token = &block.0;
            let data = &block.1;
            if token == "title" {
                lines.push(
                    format!("<h1>{}</h1>", data)
                        .to_string(),
                );
            }
            if token == "c" {
                let content_lines =
                    self.content(data);
                for content_line in content_lines {
                    lines.push(content_line)
                }
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
