use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::rest;
use nom::error::Error;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Snippet {
    Plain {
        text: Option<String>,
    },
    Kbd {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        text: Option<String>,
    },
    Link {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        text: Option<String>,
        url: Option<String>,
    },
}

pub fn snippet(source: &str) -> IResult<&str, Snippet> {
    let (remainder, captured) = alt((
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
            multispace1,
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
        rest.map(|x: &str| Snippet::Plain {
            text: Some(x.to_string()),
        }),
    ))(source)?;
    // dbg!(captured);
    // This is for individaul sections of a block like
    // raw plain text, code, strong, links, etc...
    // dbg!(source);
    Ok((
        remainder,
        captured,
        // Snippet::Kbd {
        //     attributes: None,
        //     text: Some("weave the carpet".to_string()),
        // },
    ))
}

pub fn snippet_dev(source: &str) -> IResult<&str, Snippet> {
    let (remainder, captured) = alt((
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
        let result = snippet_dev(" >salt peanuts>https://www.example.com/> red ink");
        assert_eq!(expected, result);
    }
}
