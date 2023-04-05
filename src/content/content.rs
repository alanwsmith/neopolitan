use crate::attribute::*;
use crate::content::b::b;
use crate::content::code::code;
use crate::content::em::em;
use crate::content::i::i;
use crate::content::kbd::kbd;
use crate::content::link::link;
use crate::content::span::span;
use crate::content::strike::*;
use crate::content::strong::*;
use crate::content::sub::*;
use crate::content::sup::*;
use crate::content::u::*;
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
        tuple((tag_no_case("<<i|"), take_until(">>"), tag(">>"))).map(|t| i(t).unwrap().1),
        tuple((tag_no_case("<<kbd|"), take_until(">>"), tag(">>"))).map(|t| kbd(t).unwrap().1),
        tuple((tag_no_case("<<span|"), take_until(">>"), tag(">>"))).map(|t| span(t).unwrap().1),
        tuple((tag_no_case("<<strike|"), take_until(">>"), tag(">>")))
            .map(|t| strike(t).unwrap().1),
        tuple((tag_no_case("<<strong|"), take_until(">>"), tag(">>")))
            .map(|t| strong(t).unwrap().1),
        tuple((tag_no_case("<<sub|"), take_until(">>"), tag(">>"))).map(|t| sub(t).unwrap().1),
        tuple((tag_no_case("<<sup|"), take_until(">>"), tag(">>"))).map(|t| sup(t).unwrap().1),
        tuple((tag_no_case("<<u|"), take_until(">>"), tag(">>"))).map(|t| u(t).unwrap().1),
        multispace1.map(|_| Content::Space),
        take_till(|c| c == ' ' || c == '\n' || c == '\t').map(|t: &str| Content::Text {
            text: Some(t.to_string()),
        }),
    ))(source)?;
    Ok((remainder, content))
}
