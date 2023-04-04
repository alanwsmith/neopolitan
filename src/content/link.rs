#![allow(unused_imports)]
use crate::content::content::Content;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_till;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace1;
use nom::error::Error;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

pub fn link<'a>(
    source: (&'a str, &'a str, &'a str, &'a str, &'a str),
) -> IResult<&'a str, Content> {
    Ok((
        "",
        Content::Link {
            attributes: None,
            url: source.1.to_string(),
            text: source.3.to_string(),
        },
    ))
}
