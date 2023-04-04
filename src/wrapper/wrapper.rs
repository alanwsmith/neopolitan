use crate::enums::*;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_till;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::opt;
use nom::combinator::rest;
use nom::error::Error;
use nom::multi::many0;
use nom::multi::many_till;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

#[test]
fn test_section_attribute() {
    let lines = ["class: highlighted", ""].join("\n");
    let source = lines.as_str();
    let expected = Ok((
        "",
        SectionAttribute::Attribute {
            key: Some("class".to_string()),
            value: Some("highlighted".to_string()),
        },
    ));
    let result = section_attribute(source);
    assert_eq!(expected, result);
}

pub fn section_attribute(source: &str) -> IResult<&str, SectionAttribute> {
    let (a, b) = opt(tuple((
        is_not(":"),
        tag(":"),
        space1,
        not_line_ending,
        line_ending,
    )))(source)?;
    Ok((
        a,
        SectionAttribute::Attribute {
            key: Some(b.unwrap().0.to_string()),
            value: Some(b.unwrap().3.to_string()),
        },
    ))
}

pub fn section(source: &str) -> IResult<&str, Section> {
    // dbg!(source);
    let (source, _) = multispace0(source)?;
    let (remainder, sec) = alt((
        tuple((tag("-> title\n"), alt((take_until("\n\n-> "), rest)))).map(|t| {
            let (s, att_capture) = many0(preceded(tag(">> "), section_attribute))(t.1).unwrap();
            let attributes = if att_capture.is_empty() {
                None
            } else {
                Some(att_capture)
            };
            // still not sure this is the right way to go about this.
            let (a, _) = multispace0::<&str, Error<&str>>(s).unwrap();
            let (_, b) = many_till(block, eof)(a).unwrap();
            if b.0.is_empty() {
                Section::Title {
                    attributes,
                    children: None,
                }
            } else {
                Section::Title {
                    attributes,
                    children: Some(b.0),
                }
            }
        }),
        tuple((tag("-> p\n\n"), alt((take_until("\n\n-> "), rest)))).map(|t| {
            let (_, b) = many_till(block, eof)(t.1).unwrap();
            if b.0.is_empty() {
                Section::Paragraphs {
                    attributes: None,
                    children: None,
                }
            } else {
                Section::Paragraphs {
                    attributes: None,
                    children: Some(b.0),
                }
            }
        }),
    ))(source)?;
    Ok((remainder, sec))
}

pub fn block(source: &str) -> IResult<&str, Block> {
    // dbg!(source);
    let (remainder, content) = many_till(content, alt((tag("\n\n"), eof)))(source)?;
    Ok((
        remainder,
        Block::P {
            children: Some(content.0),
        },
    ))
}

pub fn content(source: &str) -> IResult<&str, Content> {
    // dbg!(source);
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
