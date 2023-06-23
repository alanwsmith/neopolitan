use crate::source_file::SourceFile;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

impl SourceFile {
    pub fn title(&self) -> Option<String> {
        self.parse_title(&self.source_data).ok().map(|(_, b)| b)
    }

    pub fn parse_title<'a>(&'a self, source: &'a str) -> IResult<&str, String> {
        let (a, _) = take_until("-> title")(source)?;
        let (a, _) = tag("-> title")(a)?;
        let (a, _) = multispace0(a)?;
        let (a, b) = many_till(
            take(1u32),
            alt((tuple((line_ending, line_ending)).map(|_| ""), eof)),
        )(a)?;
        Ok((a, b.0.join("")))
    }
}

#[cfg(test)]

mod test {

    use std::path::PathBuf;

    use crate::source_file::SourceFile;

    #[test]
    pub fn title_with_other_sections() {
        let lines = vec![
            "-> title",
            "",
            "Foxtrot Golf",
            "",
            "-> p",
            "",
            "Hotel India",
        ];
        let sf = SourceFile {
            source_data: lines.join("\n"),
            source_path: PathBuf::from(""),
        };

        assert_eq!(Some(String::from("Foxtrot Golf")), sf.title());
    }

    #[test]
    pub fn title_with_following_content() {
        let lines = vec!["-> title", "", "Delta Echo", "", "Whiskey Tango"];
        let sf = SourceFile {
            source_data: lines.join("\n"),
            source_path: PathBuf::from(""),
        };
        assert_eq!(Some(String::from("Delta Echo")), sf.title());
    }
}
