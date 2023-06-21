use crate::source_file::source_file::SourceFile;
use nom::bytes::complete::tag;
// use nom::bytes::complete::take_till;
use nom::bytes::complete::take_until;
// use nom::combinator::eof;
use nom::combinator::rest;
use nom::IResult;

impl SourceFile {
    pub fn content(&self) -> Option<String> {
        //let (_, b) = self.parse_content().unwrap();
        Some("<p>This is a test run of the website builder</p>".to_string())
        // self.content_dev()
    }

    pub fn content_dev(&self) -> Option<String> {
        let (_, b) = self.parse_content().unwrap();
        Some(b)
    }

    fn parse_content(&self) -> IResult<&str, String> {
        let (a, _) = take_until("-> p\n\n")(self.source_data.as_ref().unwrap().as_str())?;
        let (a, _) = tag("-> p\n\n")(a)?;
        let (_, b) = rest(a)?;
        let mut content = String::from("<p>");
        content.push_str(b);
        content.push_str("</p>");
        Ok(("", content))
    }
}

#[cfg(test)]

mod test {
    use crate::source_file::source_file::SourceFile;

    #[test]
    pub fn test_first_paragraph() {
        let mut sf = SourceFile::new();
        let lines = vec!["-> p", "", "This is a test run of the website builder"];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(
            sf.content_dev(),
            Some(String::from(
                "<p>This is a test run of the website builder</p>"
            ))
        );
    }
}
