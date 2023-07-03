#![allow(unused_imports)]
use crate::parsers::neo_attribute::neo_attribute;
use crate::parsers::neo_attribute::NeoAttribute;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq)]
pub struct NeoTag {
    attributes: Vec<NeoAttribute>,
    content: String,
    tag_name: String,
}

pub fn css_class_name(
    source: &str,
) -> IResult<&str, String> {
    alpha1(source).map(|x| (x.0, x.1.to_string()))
}

pub fn neo_tag(source: &str) -> IResult<&str, NeoTag> {
    let (source, ((content, tag_name), attrs)) = delimited(
        tag("<<"),
        pair(
            separated_pair(alpha1, tag("|"), alpha1),
            opt(preceded(
                tag("|"),
                separated_list1(tag("|"), neo_attribute),
            )),
        ),
        tag(">>"),
    )(
        source
    )?;

    let nt = NeoTag {
        attributes: attrs.unwrap_or(vec![]),
        content: content.to_string(),
        tag_name: tag_name.to_string(),
    };
    Ok((source, nt))
}

#[cfg(test)]

mod test {

    use super::*;

    #[test]
    pub fn basic_neo_tag() {
        assert_eq!(
            neo_tag("<<alfa|strong>>").unwrap(),
            (
                "",
                NeoTag {
                    attributes: vec![],
                    content: "alfa".to_string(),
                    tag_name: "strong".to_string(),
                }
            )
        );
    }

    #[test]
    pub fn neotag_with_attribute() {
        assert_eq!(
            neo_tag("<<alfa|strong|class: highlight>>")
                .unwrap(),
            (
                "",
                NeoTag {
                    attributes: vec![NeoAttribute::Class(
                        vec!["highlight".to_string()]
                    )],
                    content: "alfa".to_string(),
                    tag_name: "strong".to_string(),
                }
            )
        );
    }

    //
}
