use crate::enums::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_till;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::error::Error;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

pub fn section(source: &str) -> IResult<&str, Section> {
    dbg!(source);
    let (source, _) = multispace0(source)?;
    let (remainder, sec) = alt((
        tuple((tag("-> title\n\n"), alt((take_until("\n\n-> "), rest)))).map(|t| {
            let (_, b) = many_till(block, eof)(t.1).unwrap();
            Section::Title { children: b.0 }
        }),
        tuple((tag("-> p\n\n"), alt((take_until("\n\n-> "), rest)))).map(|t| {
            let (_, b) = many_till(block, eof)(t.1).unwrap();
            Section::Paragraphs { children: b.0 }
        }),
    ))(source)?;
    Ok((remainder, sec))
}

pub fn block(source: &str) -> IResult<&str, Block> {
    dbg!(source);
    let (remainder, content) = many_till(content, alt((tag("\n\n"), eof)))(source)?;
    Ok((
        remainder,
        Block::P {
            children: content.0,
        },
    ))
}

pub fn content(source: &str) -> IResult<&str, Content> {
    dbg!(source);
    let (remainder, content) = alt((
        tuple((
            // I'm not sure if this is the right way to
            // setup the type as &str, but it works so far.
            tag_no_case::<&str, &str, Error<&str>>("<<link|"),
            take_until("|"),
            tag("|"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|t| Content::Link {
            url: t.1.to_string(),
            text: t.3.to_string(),
        }),
        multispace1.map(|_| Content::Space),
        take_till(|c| c == ' ' || c == '\n' || c == '\t').map(|t: &str| Content::Text {
            text: t.to_string(),
        }),
    ))(source)?;
    Ok((remainder, content))
}
