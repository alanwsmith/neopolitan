#![allow(unused_imports)]
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
pub enum NeoAttribute {
    Class(Vec<String>),
    Id(String),
}

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

pub fn neo_attribute(
    source: &str,
) -> IResult<&str, NeoAttribute> {
    let (source, attr_key) = terminated(
        alt((tag("class"), tag("id"))),
        tag(": "),
    )(source)?;

    match attr_key {
        "class" => {
            let (source, attr_values) =
                separated_list1(space1, css_class_name)(
                    source,
                )?;
            Ok((source, NeoAttribute::Class(attr_values)))
        }
        "id" => {
            let (source, attr_value) = alpha1(source)?;
            Ok((
                source,
                NeoAttribute::Id(attr_value.to_string()),
            ))
        }
        _ => panic!("AAAAAAAAAAAAAAAAAAAAAAAAAA"),
    }
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
    pub fn solo_basic_neo_tag() {
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
    pub fn solo_neotag_with_class() {
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

    #[test]
    pub fn solo_neotag_with_multiple_classes() {
        assert_eq!(
            neo_tag("<<bravo|strong|class: bgblue text>>")
                .unwrap(),
            (
                "",
                NeoTag {
                    attributes: vec![NeoAttribute::Class(
                        vec![
                            "bgblue".to_string(),
                            "text".to_string(),
                        ]
                    )],
                    content: "bravo".to_string(),
                    tag_name: "strong".to_string(),
                }
            )
        );
    }

    #[test]
    #[ignore]
    pub fn tktktktkt_more_attribute_names() {
        // TODO: Add all the attribute names you want to use
    }

    #[test]
    #[ignore]
    pub fn tktktktkt_attributes_() {
        // TODO:
        // Setup for data-* generic names in attributes
    }

    #[test]
    #[ignore]
    pub fn solo_neotag_with_multiple_classes_with_weird_names(
    ) {
        // TODO: Setup this based off the CSS spec with the
        // differetn ways class names can be formed
        assert_eq!(
            neo_tag("<<bravo|strong|class: TKTKTKTKTK>>")
                .unwrap(),
            (
                "",
                NeoTag {
                    attributes: vec![NeoAttribute::Class(
                        vec![]
                    )],
                    content: "bravo".to_string(),
                    tag_name: "strong".to_string(),
                }
            )
        );
    }

    #[test]
    pub fn solo_neotag_with_id() {
        assert_eq!(
            neo_tag("<<echo|kbd|id: foxtrot>>").unwrap(),
            (
                "",
                NeoTag {
                    attributes: vec![NeoAttribute::Id(
                        "foxtrot".to_string()
                    )],
                    content: "echo".to_string(),
                    tag_name: "kbd".to_string(),
                }
            )
        );
    }

    //
}
