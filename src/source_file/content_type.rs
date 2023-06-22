use nom::IResult;

use crate::source_file::source_file::SourceFile;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;

// NOTE This can currently be thrown off if you put
// an -> attributes section inside a code block

impl SourceFile {
    pub fn content_type(&self) -> Option<String> {
        let (_, b) = self.parse_content_type().unwrap();
        Some(b)
    }

    fn parse_content_type(&self) -> IResult<&str, String> {
        let (a, _) = take_until("\n-> attributes")(self.source_data.as_str())?;
        let (a, _) = tag("\n-> attributes")(a)?;
        let (a, _) = take_until(">> type:")(a)?;
        let (a, _) = tag(">> type:")(a)?;
        let (a, _) = multispace1(a)?;
        let (_, b) = not_line_ending(a)?;
        Ok(("", b.trim().to_string()))
    }
}

#[cfg(test)]
mod test {
    use crate::source_file::source_file::SourceFile;
    use std::path::PathBuf;

    #[test]
    pub fn basic_type_check() {
        let lines = vec!["", "-> attributes", ">> type: echo"];
        let mut sf = SourceFile {
            source_data: lines.join("\n"),
            source_path: PathBuf::from(""),
        };
        assert_eq!(sf.content_type(), Some(String::from("echo")));
    }
}
