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
//
// use nom::error::ErrorKind;
use nom::error::ParseError;
// use nom::multi::count;
// use nom::Err;
// use nom::InputIter;
use nom::InputLength;
use nom::InputTake;
// use nom::ToUsize;
use nom::lib::std::ops::RangeFrom;
use nom::FindSubstring;
use nom::Slice;

impl SourceFile {
    pub fn content(&self) -> Option<String> {
        let (_, b) = self.parse_content().unwrap();
        b
    }

    fn parse_content(&self) -> IResult<&str, Option<String>> {
        let (_, b) = many_till(
            alt((
                tuple((
                    multispace0,
                    tag_no_case("-> "),
                    alt((tuple((
                        // these don't show up in the output
                        alt((
                            tag_no_case("attributes"),
                            tag_no_case("categories"),
                            tag_no_case("hidden"),
                        )),
                        not_line_ending,
                        line_ending,
                        alt((take_until("\n\n-> "), rest)),
                    ))
                    .map(|_| Ok(("", Some("".to_string())))),)),
                )),
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
                    )),
                )),
            )),
            eof,
        )(self.source_data.as_ref().unwrap().as_str())?;
        let mut output = String::from("");
        b.0.iter()
            .for_each(|x| output.push_str(x.2.as_ref().unwrap().1.as_ref().unwrap().as_str()));
        Ok(("", Some(output)))
    }

    pub fn content_dev(&self) -> Option<String> {
        let (_, b) = self.parse_content_dev().unwrap();
        b
    }

    fn parse_content_dev(&self) -> IResult<&str, Option<String>> {
        let (_, b) = many_till(
            alt((
                tuple((
                    multispace0,
                    tag_no_case("-> "),
                    alt((tuple((
                        // these don't show up in the output
                        alt((
                            tag_no_case("attributes"),
                            tag_no_case("categories"),
                            tag_no_case("hidden"),
                        )),
                        not_line_ending,
                        line_ending,
                        alt((take_until("\n\n-> "), rest)),
                    ))
                    .map(|_| Ok(("", Some("".to_string())))),)),
                )),
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
                            self.get_section_content::<&str, &str, nom::error::Error<&str>>(),
                        ))
                        .map(|t| self.p_section(t.0)),
                    )),
                )),
            )),
            eof,
        )(self.source_data.as_ref().unwrap().as_str())?;
        let mut output = String::from("");
        b.0.iter()
            .for_each(|x| output.push_str(x.2.as_ref().unwrap().1.as_ref().unwrap().as_str()));
        Ok(("", Some(output)))
    }

    // fn get_section_content<'a>(&'a self, source: &'a str) -> IResult<&str, String> {
    //     // let (a, b) = tuple((
    //     //     not_line_ending,
    //     //     line_ending,
    //     //     alt((take_until("\n\n-> "), rest)),
    //     // ))(source)?;
    //     Ok(("", "".to_string()))
    // }

    fn get_section_content<'a, C, Input, Error: ParseError<Input>>(
        &self,
    ) -> impl Fn(Input) -> IResult<Input, Input, Error>
    where
        Input: FindSubstring<&'a str>
            + Slice<RangeFrom<usize>>
            + InputLength
            + InputTake
            + Clone
            + std::fmt::Debug, // C: ToUsize,
    {
        dbg!("-------------------------------");
        // let c = count.to_usize();
        move |i: Input| {
            // Err(_needed) => Err(Err::Error(Error::from_error_kind(i, ErrorKind::Eof))),
            // Ok(index) => Ok(i.take_split(index)),
            //

            // dbg!(&i.take(3));
            // let x = i.clone();
            // Ok((i, x))

            let (a, b) = take_until("L")(i)?;
            dbg!(&a);
            dbg!(&b);

            Ok((a, b))
            // i.parse()

            // Ok((i.take(4), i.take(3)))
            // Ok((rest(i.take(5)).unwrap().1, i.take(3)))
        }
    }

    //
}

#[cfg(test)]
mod test {
    use crate::source_file::source_file::SourceFile;

    // TODO: Add test to verify that unknown sections
    // cause panics

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
    pub fn test_multiple_sections() {
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
            sf.content_dev(),
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

    #[test]
    pub fn ignore_hidden_sections() {
        let mut sf = SourceFile::new();
        let lines = vec![
            "-> title",
            "",
            "Tango Whiskey",
            "",
            "-> hidden",
            "",
            "Pet the dog",
        ];
        sf.source_data = Some(lines.join("\n"));
        assert_eq!(
            sf.content(),
            Some(String::from(r#"<h1 class="neo-title">Tango Whiskey</h1>"#))
        );
    }

    //
}
