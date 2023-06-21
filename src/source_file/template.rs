use nom::IResult;

use crate::source_file::source_file::SourceFile;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;

// NOTE This can currently be thrown off if you put
// an -> attributes section inside a code block

impl SourceFile {
    pub fn template(&self) -> Option<String> {
        let (_, b) = self.parse_template().unwrap();
        Some(b)
    }

    fn parse_template(&self) -> IResult<&str, String> {
        let (a, _) = alt((take_until("\n-> attributes"), rest))(
            self.source_data.as_ref().unwrap().as_str(),
        )?;
        if a == "" {
            Ok(("", String::from("default.j2")))
        } else {
            let (a, _) = tag("\n-> attributes")(a)?;
            let (a, _) = alt((take_until(">> template:"), rest))(a)?;
            if a == "" {
                Ok(("", String::from("default.j2")))
            } else {
                let (a, _) = tag(">> template:")(a)?;
                let (a, _) = multispace1(a)?;
                let (_, b) = not_line_ending(a)?;
                let mut template = b.trim().to_string();
                template.push_str(".j2");
                Ok(("", template))
            }
            // Ok(("", String::from("default.j2")))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::source_file::source_file::SourceFile;

    #[test]
    pub fn basic_template_check() {
        let mut sf = SourceFile::new();
        let lines = vec!["", "-> attributes", ">> template: delta"];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(sf.template(), Some(String::from("delta.j2")));
    }

    #[test]
    pub fn default_if_no_attributes() {
        let mut sf = SourceFile::new();
        let lines = vec!["-> title", "", "no template"];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(sf.template(), Some(String::from("default.j2")));
    }

    #[test]
    pub fn default_if_no_template_in_attributes() {
        let mut sf = SourceFile::new();
        let lines = vec!["", "-> attributes", "", ">> type: no_template"];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(sf.template(), Some(String::from("default.j2")));
    }
}
