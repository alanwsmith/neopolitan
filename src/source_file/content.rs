use crate::source_file::sections::p_section::p_section;
use crate::source_file::sections::title_section::title_section;
use crate::source_file::SourceFile;
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
                    p_section(x.6).unwrap().1.unwrap()
                } else if x.3 == "title" {
                    title_section(x.6).unwrap().1.unwrap()
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
    use crate::source_file::SourceFile;
    use std::path::PathBuf;

    // NOTE: Unknown sections are simply skipped

    #[test]
    pub fn test_title() {
        let lines = vec!["-> title", "", "Delta Hotel"];
        let sf = SourceFile {
            source_data: lines.join("\n"),
            source_path: PathBuf::from(""),
        };
        assert_eq!(
            sf.content(),
            Some(String::from(r#"<hgroup><h1>Delta Hotel</h1></hgroup>"#))
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
        let sf = SourceFile {
            source_data: lines.join("\n"),
            source_path: PathBuf::from(""),
        };
        assert_eq!(
            sf.content(),
            Some(String::from(
                r#"<hgroup><h1>Echo Foxtrot</h1></hgroup><p>Light the candle</p>"#
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
        let sf = SourceFile {
            source_data: lines.join("\n"),
            source_path: PathBuf::from(""),
        };
        assert_eq!(
            sf.content(),
            Some(String::from(
                r#"<hgroup><h1>Whiskey November</h1></hgroup>"#
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
        let sf = SourceFile {
            source_data: lines.join("\n"),
            source_path: PathBuf::from(""),
        };
        assert_eq!(
            sf.content(),
            Some(String::from(r#"<hgroup><h1>Echo Oscar</h1></hgroup>"#))
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
        let sf = SourceFile {
            source_data: lines.join("\n"),
            source_path: PathBuf::from(""),
        };
        assert_eq!(
            sf.content(),
            Some(String::from(r#"<hgroup><h1>Tango Whiskey</h1></hgroup>"#))
        );
    }

    //
}
