use nom::IResult;

use crate::source_file::source_file::SourceFile;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;

// NOTE This can currently be thrown off if you put
// an -> attributes section inside a code block
//
// TODO: Update this with a default template if
// there isn't one defined

impl SourceFile {
    pub fn template(&self) -> Option<String> {
        let (_, b) = self.parse_template().unwrap();
        Some(b)
    }

    fn parse_template(&self) -> IResult<&str, String> {
        let (a, _) = take_until("\n-> attributes")(self.source_data.as_ref().unwrap().as_str())?;
        let (a, _) = tag("\n-> attributes")(a)?;
        let (a, _) = take_until(">> template:")(a)?;
        let (a, _) = tag(">> template:")(a)?;
        let (a, _) = multispace1(a)?;
        let (_, b) = not_line_ending(a)?;
        let mut template = b.trim().to_string();
        template.push_str(".j2");
        Ok(("", template))
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
}
