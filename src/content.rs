use crate::link::link;
use crate::text::text;
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
    let (a, b) = text(source)?;
    Ok((a, vec![b]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_content() {
        let source = "alfa bravo";
        let expected = Ok((
            "",
            vec![Content::Text {
                text: Some("alfa bravo".to_string()),
            }],
        ));
        let result = content(source);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_link() {
        let source = "<<link|https://bravo.example.com/|bravo link>>";
        let expected = Ok((
            "",
            vec![Content::Link {
                source: None,
                attributes: None,
                url: Some("https://bravo.example.com/".to_string()),
                text: Some("bravo link".to_string()),
            }],
        ));
        let result = content_dev(source);
        assert_eq!(expected, result);
    }
}

pub fn content_dev(source: &str) -> IResult<&str, Vec<Content>> {
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

    // dbg!(b);
    // // text(source)?;
    // Ok((
    //     "",
    //     vec![Content::Link {
    //         source: None,
    //         attributes: None,
    //         url: Some("https://www.example.com".to_string()),
    //         text: Some("alfa bravo".to_string()),
    //     }],
    // ))
}
