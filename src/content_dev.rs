#![allow(warnings)]
use crate::page_builder::PageBuilder;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_until1;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::not;
// use nom::character::complete::anychar;
use nom::branch::alt;
use nom::combinator::opt;
use nom::combinator::rest;
use nom::IResult;

impl PageBuilder {
    pub fn content_dev(
        &self, source: String,
    ) -> Vec<String> {
        let mut lines: Vec<String> = vec![];

        // lines.push(format!("<p>{}</p>", source,));

        // Don't know if I need to pull this out
        // like this or not. tbd
        let mut send_source = source.as_str();

        // match self.parse_content(send_source) {
        //     Ok((a, b)) => {
        //         dbg!(a);
        //         dbg!(b);
        //         match self.parse_content(a) {
        //             Ok((c, d)) => {
        //                 dbg!(c);
        //                 dbg!(d);
        //             }
        //             Err(e) => {
        //                 dbg!(e);
        //             }
        //         }
        //     }
        //     Err(e) => {
        //         dbg!(e);
        //     }
        // }

        while let Ok((next, content)) =
            self.parse_content(send_source)
        {
            lines.push(format!(
                "<p>{}</p>",
                content.to_string()
            ));
            send_source = next;
        }

        // lines = vec![
        //     "<p>alfa line</p>".to_string(),
        //     "<p>bravo line</p>".to_string(),
        // ];

        lines
    }

    pub fn parse_content_dev<'a>(
        &'a self, next: &'a str,
    ) -> IResult<&str, &str> {
        let (next, content) = multispace0(next)?;
        not(eof)(next)?;
        let (next, content) =
            alt((take_until1("\n\n"), rest))(next)?;
        Ok((next, content.trim()))

        // // opt(tag("\n\n"))(data)?;
        // let (data, content) =
        // alt((take_until1("\n\n"), rest))(data)?;
        // dbg!(content);
        // dbg!(data);

        // Ok((next, content))
    }
}
