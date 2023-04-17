#![allow(warnings)]
use std::vec;

use crate::attribute::*;
use crate::content::content::Content;
// use crate::section::attributes::attributes;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::rest;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

pub fn neo_tag_attributes(source: &str) -> IResult<&str, Option<Vec<Attribute>>> {
    let (_, parts) = separated_list0(tag("|"), is_not("|"))(source)?;
    dbg!(&parts);
    if parts.len() > 1 {
        let attributes: Option<Vec<Attribute>> = Some(
            parts
                .iter()
                .skip(1)
                .map(|a| attribute(a).unwrap().1)
                .collect(),
        );
        Ok(("", attributes))
    } else {
        Ok(("", None))
    }
}

pub fn neo_tag<'a>(source: (&'a str, &'a str, &'a str)) -> IResult<&'a str, Content> {
    let (_, parts) = tuple((take_until("|"), tag("|"), rest))(source.1)?;
    // let attributes = neo_tag_attributes(parts.2).unwrap().1;
    // dbg!(attributes);
    let (a, b) = alt((
        tuple((multispace0, tag("strong"), multispace0)).map(|x| Content::Strong {
            //attributes: attributes,
            attributes: neo_tag_attributes(parts.2).unwrap().1,
            text: Some(parts.0.to_string()),
        }),
        tuple((multispace0, tag("strong"), multispace0)).map(|x| Content::Strong {
            attributes: None,
            text: None,
        }),
    ))(parts.2)?;

    Ok((a, b))

    // // dbg!(a);
    // dbg!(&parts.0);
    // dbg!(&parts);

    // // dbg!(a);
    // dbg!(b);

    // let (_, parts) = separated_list0(tag("|"), is_not("|"))(source.1)?;
    // let (a, b) = alt((
    //     tuple((multispace0, tag("strong"), multispace0)).map(|x| Content::Strong {
    //         attributes: neo_tag_attributes(),
    //         text: Some(parts[0].trim().to_string()),
    //     }),
    //     tuple((multispace0, tag("strong"), multispace0)).map(|x| Content::Strong {
    //         attributes: None,
    //         text: None,
    //     }),
    // ))(parts[1])?;
    // dbg!(b);

    // // get the text content
    // let (a, text) = take_until("|")(source.1)?;
    // let (a, b) = tag("|")(a)?;
    // dbg!(a);
    // // Ok(("", Content::Placeholder))

    // Ok((
    //     "",
    //     Content::Strong {
    //         attributes: Some(vec![Attribute::Basic {
    //             key: Some("class".to_string()),
    //             value: Some("delta".to_string()),
    //         }]),
    //         text: Some("bravo".to_string()),
    //     },
    // ))
}

#[cfg(test)]
mod test {

    use crate::attribute::*;
    use crate::block::block::*;
    use crate::content::content::*;
    use crate::parse::parse;
    use crate::section::section::*;
    use crate::wrapper::wrapper::*;

    #[test]
    fn basic_neo_tag() {
        let lines = vec!["-> p", "", "alfa <<bravo|strong|class: delta>> charlie"].join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::Paragraphs {
                attributes: None,
                children: Some(vec![Block::P {
                    children: Some(vec![
                        Content::Text {
                            text: Some("alfa".to_string()),
                        },
                        Content::Space,
                        Content::Strong {
                            attributes: Some(vec![Attribute::Basic {
                                key: Some("class".to_string()),
                                value: Some("delta".to_string()),
                            }]),
                            text: Some("bravo".to_string()),
                        },
                        Content::Space,
                        Content::Text {
                            text: Some("charlie".to_string()),
                        },
                    ]),
                }]),
            }]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }
}
