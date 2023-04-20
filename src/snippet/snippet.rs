use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::rest;
use nom::error::Error;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Snippet {
    Abbr {
        attributes: Option<Vec<SnippetAttribute>>,
        text: Option<String>,
    },
    Kbd {
        attributes: Option<Vec<SnippetAttribute>>,
        text: Option<String>,
    },
    Link {
        attributes: Option<Vec<SnippetAttribute>>,
        text: Option<String>,
        url: Option<String>,
    },
    Plain {
        text: Option<String>,
    },
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum SnippetAttribute {
    Attribute {
        key: Option<String>,
        value: Option<String>,
    },
}

pub fn inline_attributes(source: &str) -> IResult<&str, Option<Vec<SnippetAttribute>>> {
    let (_a, b) = separated_list0(tag("|"), alt((take_until("|"), rest)))(source)?;
    let (_a, b) = separated_list0(tag(":"), alt((take_until(":"), rest)))(b[1])?;
    Ok((
        "",
        Some(vec![SnippetAttribute::Attribute {
            key: Some(b[0].trim().to_string()),
            value: Some(b[1].trim().to_string()),
        }]),
    ))
}

pub fn snippet(source: &str) -> IResult<&str, Snippet> {
    let (remainder, captured) = alt((
        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            is_not("|"),
            tag("|"),
            multispace0,
            tag("abbr"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| {
            dbg!(x.6);
            Snippet::Abbr {
                attributes: inline_attributes(x.6).unwrap().1,
                text: Some(x.2.to_string()),
            }
        }),
        tuple((
            multispace1::<&str, Error<&str>>,
            tag("<<"),
            is_not("|"),
            tag("|"),
            multispace0,
            tag("kbd"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|x| Snippet::Kbd {
            attributes: None,
            text: Some(x.2.to_string()),
        }),
        tuple((
            multispace1::<&str, Error<&str>>,
            tag(">"),
            is_not(">"),
            tag(">"),
            is_not(">"),
            tag(">"),
        ))
        .map(|x| Snippet::Link {
            attributes: None,
            text: Some(x.2.to_string()),
            url: Some(x.4.to_string()),
        }),
        take_until(" <<").map(|x: &str| Snippet::Plain {
            text: Some(x.to_string()),
        }),
        rest.map(|x: &str| Snippet::Plain {
            text: Some(x.to_string()),
        }),
    ))(source)?;
    Ok((remainder, captured))
}

#[cfg(test)]
mod test {
    use crate::snippet::snippet::*;

    #[test]
    pub fn basic_test() {
        let expected = Snippet::Plain {
            text: Some("Take the winding path".to_string()),
        };
        let result = snippet("Take the winding path").unwrap().1;
        assert_eq!(expected, result);
    }

    #[test]
    pub fn basic_kbd_test() {
        let expected = Ok((
            " with more words",
            Snippet::Kbd {
                attributes: None,
                text: Some("weave the carpet".to_string()),
            },
        ));
        let result = snippet(" <<weave the carpet|kbd>> with more words");
        assert_eq!(expected, result);
    }

    #[test]
    pub fn shorthand_link_test() {
        let expected = Ok((
            " red ink",
            Snippet::Link {
                attributes: None,
                url: Some("https://www.example.com/".to_string()),
                text: Some("salt peanuts".to_string()),
            },
        ));
        let result = snippet(" >salt peanuts>https://www.example.com/> red ink");
        assert_eq!(expected, result);
    }

    #[test]
    pub fn text_stop_at_tag() {
        let expected = Ok((
            " <<best|kbd>> compass",
            Snippet::Plain {
                text: Some("Bring your".to_string()),
            },
        ));
        let result = snippet("Bring your <<best|kbd>> compass");
        assert_eq!(expected, result);
    }

    #[test]
    pub fn abbr_with_attribute() {
        let source = " <<weave the carpet|abbr|class: echo>>";
        let expected = Ok((
            "",
            Snippet::Abbr {
                attributes: Some(vec![SnippetAttribute::Attribute {
                    key: Some("class".to_string()),
                    value: Some("echo".to_string()),
                }]),
                text: Some("weave the carpet".to_string()),
            },
        ));
        let result = snippet(source);
        assert_eq!(expected, result);
    }
}
