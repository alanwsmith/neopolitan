#![allow(warnings)]
use crate::page_builder::PageBuilder;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_until1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
use nom::error::Error;
use nom::error::ParseError;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;

impl PageBuilder {
    pub fn split(
        &self, source: &str,
    ) -> Vec<(String, String)> {
        //
        // This is for spacing so I can see what's going on
        //
        let mut lines: Vec<(String, String)> =
            vec![];

        let mut send_source = source.clone();

        while let Ok((next, (token, content))) =
            self.get_blocks(send_source)
        {
            lines.push((
                token.to_string(),
                content.trim().to_string(),
            ));
            send_source = next;
        }
        lines
    }

    pub fn get_blocks<'a>(
        &'a self, data: &'a str,
    ) -> IResult<&str, (&str, &str)> {
        let (data, _) = multispace0(data)?;
        let (data, _) = tag("-> ")(data)?;
        let (data, token) = alphanumeric1(data)?;
        let (data, _) = not_line_ending(data)?;
        let (data, _) = line_ending(data)?;
        let (data, _) = line_ending(data)?;
        let (data, content) = alt((
            take_until1("\n\n-> "),
            rest,
        ))(data)?;
        Ok((data, (token, content)))
    }
}
