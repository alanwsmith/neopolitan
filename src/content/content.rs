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

#[derive(Debug, PartialEq)]
pub enum Content {
    Link { text: String, url: String },
    Text { text: String },
    Space,
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
