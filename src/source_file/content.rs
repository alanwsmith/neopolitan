use crate::source_file::source_file::SourceFile;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
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
                tag_no_case("-> "),
                alt((
                    tuple((
                        tag_no_case("title"),
                        not_line_ending,
                        line_ending,
                        alt((take_until("\n\n-> "), rest)),
                    ))
                    .map(|t| self.title_section(t.3)),
                    tuple((
                        tag_no_case("p"),
                        not_line_ending,
                        line_ending,
                        alt((take_until("\n\n-> "), rest)),
                    ))
                    .map(|t| self.p_section(t.3)),
                    tuple((
                        tag_no_case("categories"),
                        not_line_ending,
                        line_ending,
                        alt((take_until("\n\n-> "), rest)),
                    ))
                    .map(|_| Ok(("", Some("".to_string())))),
                    tuple((
                        tag_no_case("attributes"),
                        not_line_ending,
                        line_ending,
                        alt((take_until("\n\n-> "), rest)),
                    ))
                    .map(|_| Ok(("", Some("".to_string())))),
                    // When all section types are in place this
                    // rest.map should be able to be removed. Right
                    // now it's just catching things that haven't been
                    // defined yet
                    rest.map(|x| Ok(("", Some(String::from(x))))),
                )),
            )),
            eof,
        )(self.source_data.as_ref().unwrap().as_str())?;
        let mut output = String::from("");
        b.0.iter()
            .for_each(|x| output.push_str(x.2.as_ref().unwrap().1.as_ref().unwrap().as_str()));
        Ok(("", Some(output)))
    }

    //
}

#[cfg(test)]
mod test {
    use crate::source_file::source_file::SourceFile;

    #[test]
    pub fn test_title() {
        let mut sf = SourceFile::new();
        let lines = vec!["-> title", "", "Delta Hotel"];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(
            sf.content(),
            Some(String::from(r#"<h1 class="neo-title">Delta Hotel</h1>"#))
        );
    }

    #[test]
    pub fn test_multiple_section() {
        let mut sf = SourceFile::new();
        let lines = vec![
            "-> title",
            "",
            "Echo Foxtrot",
            "",
            "-> p",
            "",
            "Light the candle",
        ];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(
            sf.content(),
            Some(String::from(
                r#"<h1 class="neo-title">Echo Foxtrot</h1><p>Light the candle</p>"#
            ))
        );
    }

    #[test]
    pub fn ignore_categories() {
        let mut sf = SourceFile::new();
        let lines = vec![
            "-> title",
            "",
            "Whiskey November",
            "",
            "-> categories",
            ">> Example",
        ];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(
            sf.content(),
            Some(String::from(
                r#"<h1 class="neo-title">Whiskey November</h1>"#
            ))
        );
    }

    #[test]
    pub fn ignore_attributes() {
        let mut sf = SourceFile::new();
        let lines = vec![
            "-> title",
            "",
            "Echo Oscar",
            "",
            "-> attributes",
            ">> Example",
        ];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(
            sf.content(),
            Some(String::from(r#"<h1 class="neo-title">Echo Oscar</h1>"#))
        );
    }

    //
}
