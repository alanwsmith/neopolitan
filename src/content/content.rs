use crate::content::link::link;
// use crate::content::text::text;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::combinator::rest;
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Content {
    Text {
        text: Option<String>,
    },
    Link {
        source: Option<String>,
        attributes: Option<HashMap<String, String>>,
        url: Option<String>,
        text: Option<String>,
    },
}

pub fn content(source: &str) -> IResult<&str, Vec<Content>> {
    let (a, b) = alt((
        tag_no_case("<<link|").map(|_| Content::Link {
            source: None,
            attributes: None,
            url: None,
            text: None,
        }),
        rest.map(|x: &str| Content::Text {
            text: Some(x.to_string()),
        }),
    ))(source)
    .map(|(a, b)| match b {
        Content::Text { text: _ } => (a, b),
        Content::Link {
            source: _,
            attributes: _,
            url: _,
            text: _,
        } => link(a).unwrap(),
    })?;
    Ok((a, vec![b]))
}

pub fn content_dev(source: &str) -> IResult<&str, Content> {
    let (a, b) = alt((
        tag_no_case("<<link|").map(|_| Content::Link {
            source: None,
            attributes: None,
            url: None,
            text: None,
        }),
        rest.map(|x: &str| Content::Text {
            text: Some(x.to_string()),
        }),
    ))(source)
    .map(|(a, b)| match b {
        Content::Text { text: _ } => (a, b),
        Content::Link {
            source: _,
            attributes: _,
            url: _,
            text: _,
        } => link(a).unwrap(),
    })?;

    Ok((a, b))

    // Ok((
    //     a,
    //     Content::Text {
    //         text: Some("the ".to_string()),
    //     },
    // ))

    // Ok((
    //     a,
    //     vec![
    //         Content::Text {
    //             text: Some("the ".to_string()),
    //         },
    //         Content::Link {
    //             source: None,
    //             attributes: None,
    //             url: Some("localhost:9090".to_string()),
    //             text: Some("the ".to_string()),
    //         },
    //         Content::Text {
    //             text: Some(" plank".to_string()),
    //         },
    //     ],
    // ))
}
