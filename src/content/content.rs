use crate::attribute::*;
// use crate::content::b::b;
// use crate::content::code::code;
use crate::content::code_shorthand::*;
use crate::content::em_shorthand::em_shorthand;
use crate::content::strong_shorthand::strong_shorthand;
// use crate::content::em::em;
// use crate::content::i::i;
// use crate::content::kbd::kbd;
// use crate::content::link::link;
use crate::content::neo_tag::neo_tag;
// use crate::content::span::span;
// use crate::content::strike::*;
// use crate::content::strong::*;
// use crate::content::sub::*;
// use crate::content::sup::*;
// use crate::content::u::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_till;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace1;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Content {
    B {
        attributes: Option<Vec<Attribute>>,
        text: Option<String>,
    },
    Code {
        attributes: Option<Vec<Attribute>>,
        text: Option<String>,
    },
    CodeShorthand {
        attributes: Option<Vec<Attribute>>,
        text: Option<String>,
    },
    Em {
        attributes: Option<Vec<Attribute>>,
        text: Option<String>,
    },
    EmShorthand {
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
    StrongShorthand {
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
    Placeholder,
}

pub fn content(source: &str) -> IResult<&str, Content> {
    let (remainder, content) = alt((
        tuple((tag_no_case("<<"), take_until(">>"), tag(">>"))).map(|t| neo_tag(t).unwrap().1),
        tuple((
            tag_no_case("`"),
            take_until("`"),
            tag("`"),
            take_until("`"),
            tag("`"),
        ))
        .map(|t| code_shorthand(t).unwrap().1),

        tuple((
            tag_no_case("_"),
            take_until("_"),
            tag("_"),
            take_until("_"),
            tag("_"),
        ))
        .map(|t| em_shorthand(t).unwrap().1),

        tuple((
            tag_no_case("*"),
            take_until("*"),
            tag("*"),
            take_until("*"),
            tag("*"),
        ))
        .map(|t| strong_shorthand(t).unwrap().1),



        multispace1.map(|_| Content::Space),
        take_till(|c| c == ' ' || c == '\n' || c == '\t').map(|t: &str| Content::Text {
            text: Some(t.to_string()),
        }),
    ))(source)?;
    Ok((remainder, content))
}
