use crate::source_file::source_file::SourceFile;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

impl SourceFile {
    // pub fn title_old(&self) -> Option<String> {
    //     // self.title_dev()
    //     // let (_, b) = self
    //     //     .parse_title(&self.source_data.as_ref().unwrap())
    //     //     .unwrap();
    //     // Some(b)
    //     None
    // }

    pub fn title(&self) -> Option<String> {
        let (_, b) = self
            .parse_title(&self.source_data.as_ref().unwrap())
            .unwrap();
        Some(b)
    }

    // pub fn parse_title_old<'a>(&'a self, source: &'a str) -> IResult<&str, String> {
    //     let (a, _) = take_until("-> title")(source)?;
    //     let (a, _) = tag("-> title")(a)?;
    //     let (a, _) = multispace1(a)?;
    //     let (a, b) = take_until("\n->")(a)?;
    //     Ok((a, b.trim().to_string()))
    // }

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

    use crate::source_file::source_file::SourceFile;

    #[test]
    pub fn title_with_other_sections() {
        let mut sf = SourceFile::new();
        let lines = vec![
            "-> title",
            "",
            "Foxtrot Golf",
            "",
            "-> p",
            "",
            "Hotel India",
        ];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(Some(String::from("Foxtrot Golf")), sf.title());
    }

    #[test]
    pub fn title_with_following_content() {
        let mut sf = SourceFile::new();
        let lines = vec!["-> title", "", "Delta Echo", "", "Whiskey Tango"];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(Some(String::from("Delta Echo")), sf.title());
    }
}
