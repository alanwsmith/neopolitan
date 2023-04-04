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
    // // dbg!(source);
    // let (_, content) = alt((
    //     tuple((
    //         // I'm not sure if this is the right way to
    //         // setup the type as &str, but it works so far.
    //         tag_no_case::<&str, &str, Error<&str>>("<<link|"),
    //         take_until("|"),
    //         tag("|"),
    //         take_until(">>"),
    //         tag(">>"),
    //     ))
    //     .map(|t| Content::Link {
    //         url: t.1.to_string(),
    //         text: t.3.to_string(),
    //     }),
    //     multispace1.map(|_| Content::Space),
    //     take_till(|c| c == ' ' || c == '\n' || c == '\t').map(|t: &str| Content::Text {
    //         text: t.to_string(),
    //     }),
    // ))(source)?;

    Ok((
        "",
        Content::Link {
            url: source.1.to_string(),
            text: source.3.to_string(),
        },
    ))
}
