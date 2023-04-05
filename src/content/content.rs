use crate::attribute::*;
use crate::content::b::b;
use crate::content::code::code;
use crate::content::em::em;
use crate::content::link::link;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_till;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace1;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq)]
pub enum Content {
    B {
        attributes: Option<Vec<Attribute>>,
        text: Option<String>,
    },
    Code {
        attributes: Option<Vec<Attribute>>,
        text: Option<String>,
    },
    Em {
        attributes: Option<Vec<Attribute>>,
        text: Option<String>,
    },
    I {
        attributes: Option<Vec<Attribute>>,
        text: Option<String>,
    },
    Kbd {
        attributes: Option<Vec<Attribute>>,
        text: Option<String>,
    },
    Link {
        attributes: Option<Vec<Attribute>>,
        text: Option<String>,
        url: Option<String>,
    },
    Space,
    Span {
        attributes: Option<Vec<Attribute>>,
        text: Option<String>,
    },
    Strike {
        attributes: Option<Vec<Attribute>>,
        text: Option<String>,
    },
    Strong {
        attributes: Option<Vec<Attribute>>,
        text: Option<String>,
    },
    Sub {
        attributes: Option<Vec<Attribute>>,
        text: Option<String>,
    },
    Sup {
        attributes: Option<Vec<Attribute>>,
        text: Option<String>,
    },
    Text {
        text: Option<String>,
    },
    U {
        attributes: Option<Vec<Attribute>>,
        text: Option<String>,
    },
}

pub fn content(source: &str) -> IResult<&str, Content> {
    // dbg!(source);
    let (remainder, content) = alt((
        tuple((tag_no_case("<<b|"), take_until(">>"), tag(">>"))).map(|t| b(t).unwrap().1),
        tuple((tag_no_case("<<code|"), take_until(">>"), tag(">>"))).map(|t| code(t).unwrap().1),
        tuple((
            tag_no_case("<<link|"),
            take_until("|"),
            tag("|"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|t| link(t).unwrap().1),
        tuple((tag_no_case("<<em|"), take_until(">>"), tag(">>"))).map(|t| em(t).unwrap().1),
        multispace1.map(|_| Content::Space),
        take_till(|c| c == ' ' || c == '\n' || c == '\t').map(|t: &str| Content::Text {
            text: Some(t.to_string()),
        }),
    ))(source)?;
    Ok((remainder, content))
}
