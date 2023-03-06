use crate::page_builder::PageBuilder;
use nom::branch::alt;
// use nom::bytes::complete::tag;
// use nom::bytes::complete::take_until;
use nom::bytes::complete::take_until1;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::not;
// use nom::combinator::opt;
use nom::combinator::rest;
use nom::IResult;

impl PageBuilder {
    pub fn content(
        &self, source: String,
    ) -> Vec<String> {
        let mut lines: Vec<String> = vec![];
        let mut send_source = source.as_str();
        while let Ok((next, content)) =
            self.parse_content(send_source)
        {
            lines.push(format!(
                "<p>{}</p>",
                content.to_string()
            ));
            send_source = next;
        }
        lines
    }

    pub fn parse_content<'a>(
        &'a self, next: &'a str,
    ) -> IResult<&str, &str> {
        let (next, _) = multispace0(next)?;
        not(eof)(next)?;
        let (next, content) =
            alt((take_until1("\n\n"), rest))(next)?;
        Ok((next, content.trim()))
    }
}
