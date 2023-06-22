use crate::source_file::source_file::SourceFile;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

impl SourceFile {
    pub fn content(&self) -> Option<String> {
        let (_, b) = self.parse_content().unwrap();
        b
    }

    fn parse_content(&self) -> IResult<&str, Option<String>> {
        let (_, b) = many_till(
            tuple((
                multispace0,
                tag("->"),
                multispace1,
                not_line_ending,
                line_ending,
                multispace0,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|x| {
                if x.3 == "p" {
                    self.p_section(x.6).unwrap().1.unwrap()
                    // self.parse_block(self.p_section(x.6).unwrap().1.unwrap().as_str())
                    //     .unwrap()
                    //     .1
                    //     .unwrap()
                } else if x.3 == "title" {
                    self.title_section(x.6).unwrap().1.unwrap()
                } else {
                    "".to_string()
                }
            }),
            eof,
        )(self.source_data.as_str())?;
        let mut content = "".to_string();
        b.0.iter().for_each(|sec| content.push_str(sec));
        Ok(("", Some(content)))
    }

    //
}

#[cfg(test)]
mod test {
    use crate::source_file::source_file::SourceFile;
    use std::path::PathBuf;

    // NOTE: Unknown sections are simply skipped

    #[test]
    pub fn test_title() {
        let lines = vec!["-> title", "", "Delta Hotel"];
        let mut sf = SourceFile {
            source_data: lines.join("\n"),
            source_path: PathBuf::from(""),
        };
        assert_eq!(
            sf.content(),
            Some(String::from(r#"<h1 class="neo-title">Delta Hotel</h1>"#))
        );
    }

    #[test]
    pub fn test_multiple_sections() {
        let lines = vec![
            "-> title",
            "",
            "Echo Foxtrot",
            "",
            "-> p",
            "",
            "Light the candle",
        ];
        let mut sf = SourceFile {
            source_data: lines.join("\n"),
            source_path: PathBuf::from(""),
        };
        assert_eq!(
            sf.content(),
            Some(String::from(
                r#"<h1 class="neo-title">Echo Foxtrot</h1><p>Light the candle</p>"#
            ))
        );
    }

    #[test]
    pub fn ignore_categories() {
        let lines = vec![
            "-> title",
            "",
            "Whiskey November",
            "",
            "-> categories",
            ">> Example",
        ];
        let mut sf = SourceFile {
            source_data: lines.join("\n"),
            source_path: PathBuf::from(""),
        };
        assert_eq!(
            sf.content(),
            Some(String::from(
                r#"<h1 class="neo-title">Whiskey November</h1>"#
            ))
        );
    }

    #[test]
    pub fn ignore_attributes() {
        let lines = vec![
            "-> title",
            "",
            "Echo Oscar",
            "",
            "-> attributes",
            ">> Example",
        ];
        let mut sf = SourceFile {
            source_data: lines.join("\n"),
            source_path: PathBuf::from(""),
        };
        assert_eq!(
            sf.content(),
            Some(String::from(r#"<h1 class="neo-title">Echo Oscar</h1>"#))
        );
    }

    #[test]
    pub fn ignore_hidden_sections() {
        let lines = vec![
            "-> title",
            "",
            "Tango Whiskey",
            "",
            "-> hidden",
            "",
            "Pet the dog",
        ];
        let mut sf = SourceFile {
            source_data: lines.join("\n"),
            source_path: PathBuf::from(""),
        };
        assert_eq!(
            sf.content(),
            Some(String::from(r#"<h1 class="neo-title">Tango Whiskey</h1>"#))
        );
    }

    //
}
