#![allow(unused_imports)]
use crate::parsers::neo_attribute::neo_attribute;
use crate::parsers::neo_attribute::NeoAttribute;
use crate::parsers::neo_element::neo_element;
use crate::parsers::neo_element::NeoElement;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct NeoTag {
    content: String,
    element: NeoElement,
}

pub fn css_class_name(
    source: &str,
) -> IResult<&str, String> {
    alpha1(source).map(|x| (x.0, x.1.to_string()))
}

pub fn neo_tag(source: &str) -> IResult<&str, NeoTag> {
    let (source, (content, element)) = delimited(
        tag("<<"),
        separated_pair(alpha1, tag("|"), neo_element),
        tag(">>"),
    )(source)?;
    let nt = NeoTag {
        content: content.to_string(),
        element,
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
                    content: "alfa".to_string(),
                    element: NeoElement::Strong(vec![]),
                }
            )
        );
    }

    #[test]
    pub fn neo_tag_with_class() {
        assert_eq!(
            neo_tag("<<alfa|strong|class: bravo>>")
                .unwrap(),
            (
                "",
                NeoTag {
                    content: "alfa".to_string(),
                    element: NeoElement::Strong(vec![
                        NeoAttribute::Class(vec![
                            "bravo".to_string()
                        ])
                    ]),
                }
            )
        );
    }

    #[test]
    #[ignore]
    pub fn neo_tag_with_content_including_spaces() {
        assert_eq!(
            neo_tag("<<delta 1 echo ! foxtrot|abbr|class: kilo>>")
                .unwrap(),
            (
                "",
                NeoTag {
                    content: "delta 1 echo ! foxtrot".to_string(),
                    element: NeoElement::Abbr(vec![
                        NeoAttribute::Class(vec![
                            "kilo".to_string()
                        ])
                    ]),
                }
            )
        );
    }

    //
}
