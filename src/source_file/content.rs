use crate::source_file::source_file::SourceFile;
use nom::branch::alt;
use nom::bytes::complete::tag;
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
// use nom::AsChar;
use nom::FindSubstring;
use nom::InputIter;
use nom::Slice;
use std::ops::RangeTo;

impl SourceFile {
    pub fn content_dev3(&self) -> Option<String> {
        let (_, b) = self.parse_content_dev3().unwrap();
        b
    }

    pub fn content_dev2(&self) -> Option<String> {
        let (_, b) = self.parse_content_dev2().unwrap();
        b
    }

    fn parse_content_dev3(&self) -> IResult<&str, Option<String>> {
        let (_, b) = many_till(
            tuple((
                multispace0,
                tag("->"),
                multispace0,
                not_line_ending,
                line_ending,
                multispace0,
                alt((take_until("\n\n-> "), rest)),
            ))
            .map(|x| {
                if x.3 == "p" {
                    dbg!(&x);
                    "".to_string()
                } else if x.3 == "title" {
                    dbg!(&x);
                    dbg!(self.title_section(x.6).unwrap().1.unwrap());
                    self.title_section(x.6).unwrap().1.unwrap()
                } else {
                    "".to_string()
                }
            }),
            eof,
        )(self.source_data.as_ref().unwrap().as_str())?;

        let mut content = "".to_string();
        b.0.iter().for_each(|sec| content.push_str(sec));

        Ok(("", Some(content)))
    }

    fn parse_content_dev2(&self) -> IResult<&str, Option<String>> {
        let (_, b) = many_till(
            tuple((
                multispace0,
                tag("->"),
                multispace0,
                not_line_ending,
                line_ending,
                multispace0,
                alt((take_until("\n\n-> "), rest)),
            )),
            eof,
        )(self.source_data.as_ref().unwrap().as_str())?;
        let mut content = "".to_string();
        b.0.iter().for_each(|sec| {
            if sec.3 == "p" {
                content.push_str(self.p_section(&sec.6).unwrap().1.unwrap().as_str())
            } else if sec.3 == "title" {
                content.push_str(self.title_section(&sec.6).unwrap().1.unwrap().as_str())
            }
        });
        Ok(("", Some(content)))

        // let (_, b) = many_till(
        //     alt((
        //         tuple((
        //             multispace0,
        //             tag_no_case("-> "),
        //             alt((tuple((
        //                 // these don't show up in the output
        //                 alt((
        //                     tag_no_case("attributes"),
        //                     tag_no_case("categories"),
        //                     tag_no_case("hidden"),
        //                 )),
        //                 not_line_ending,
        //                 line_ending,
        //                 alt((take_until("\n\n-> "), rest)),
        //             ))
        //             .map(|_| Ok(("", Some("".to_string())))),)),
        //         )),
        //         tuple((
        //             multispace0,
        //             tag_no_case("-> "),
        //             alt((
        //                 tuple((
        //                     tag_no_case("title"),
        //                     not_line_ending,
        //                     line_ending,
        //                     alt((take_until("\n\n-> "), rest)),
        //                 ))
        //                 .map(|t| self.title_section(t.3)),
        //                 tuple((
        //                     tag_no_case("p"),
        //                     not_line_ending,
        //                     line_ending,
        //                     alt((take_until("\n\n-> "), rest)),
        //                 ))
        //                 .map(|t| self.p_section(t.3)),
        //             )),
        //         )),
        //     )),
        //     eof,
        // )(self.source_data.as_ref().unwrap().as_str())?;
        // let mut output = String::from("");
        // b.0.iter()
        //     .for_each(|x| output.push_str(x.2.as_ref().unwrap().1.as_ref().unwrap().as_str()));

        // Ok(("", Some(output)))
        // Ok(("", Some("<h1>Delta Hotel</h1>".to_string())))
    }

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
            + Slice<std::ops::Range<usize>>
            + InputLength
            + InputIter
            + InputTake
            + Clone
            // + AsChar
            + Slice<RangeTo<usize>>
            // + nom::UnspecializedInput
            + std::fmt::Debug,
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
            // let (a, b) = not_line_ending(i)?;
            // let (a, b) = line_ending(a)?;
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
            sf.content_dev3(),
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
            sf.content_dev2(),
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
