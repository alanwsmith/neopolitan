// #![allow(warnings)]
//use nom::branch::alt;
//use nom::bytes::complete::is_not;
//use nom::bytes::complete::tag;
////use nom::bytes::complete::take_till;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace1;
// use nom::combinator::not;
use nom::multi::many0;
// use nom::multi::many_till;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::sequence::tuple;
//use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
//use nom::character::complete::space1;
//use nom::combinator::eof;
//use nom::combinator::rest;
//use nom::sequence::tuple;
use nom::IResult;
//use nom::Parser;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum SectionAttributeV2 {
    Attribute {
        key: Option<String>,
        value: Option<String>,
    },
}

// pub fn section_attributes_v2(source: &str) -> IResult<&str, Option<Vec<SectionAttributeV2>>> {
//     let (remainder, b) = many0(section_attribute_v2)(source)?;
//     if b.is_empty() {
//         Ok((remainder, None))
//     } else {
//         Ok((remainder, Some(b)))
//     }
// }

// pub fn section_attribute_v2(source: &str) -> IResult<&str, SectionAttributeV2> {
//     let (a, b) = tuple((
//         tag(">>"),
//         multispace1,
//         take_until(":"),
//         tag(":"),
//         multispace1,
//         not_line_ending,
//         line_ending,
//     ))(source)?;
//     let response = SectionAttributeV2::Attribute {
//         key: Some(b.2.to_string()),
//         value: Some(b.5.to_string()),
//     };
//     Ok((a, response))
// }

pub fn section_attributes_v2(source: &str) -> IResult<&str, Option<Vec<SectionAttributeV2>>> {
    let (remainder, b) = many0(section_attribute_v2)(source)?;
    if b.is_empty() {
        Ok((remainder, None))
    } else {
        Ok((remainder, Some(b)))
    }
}

pub fn section_attribute_v2(source: &str) -> IResult<&str, SectionAttributeV2> {
    let (remainder, _) = tag(">>")(source)?;
    let (remainder, _) = multispace1(remainder)?;
    let (remainder, captured) = not_line_ending(remainder)?;
    let (remainder, _) = line_ending(remainder)?;
    let (_, parts) = split(captured, ":")?;
    if parts.len() == 1 {
        let response = SectionAttributeV2::Attribute {
            key: Some(parts[0].trim().to_string()),
            value: None,
        };
        Ok((remainder, response))
    } else {
        let response = SectionAttributeV2::Attribute {
            key: Some(parts[0].trim().to_string()),
            value: Some(parts[1].trim().to_string()),
        };
        Ok((remainder, response))
    }
}

fn split<'a>(source: &'a str, separator: &'a str) -> IResult<&'a str, Vec<&'a str>> {
    let (remainder, _) = opt(tag(separator))(source)?;
    let (_, items) = separated_list0(tag(separator), is_not(separator))(remainder)?;
    Ok(("", items))
}

#[cfg(test)]
mod test {
    use crate::section::section_attributes_v2::SectionAttributeV2;
    use crate::section::section_attributes_v2::*;
    #[test]
    fn no_attributes() {
        let lines = ["Twist the valve"].join("\n");
        let source = lines.as_str();
        let expected = Ok(("Twist the valve", None));
        let result = section_attributes_v2(source);
        assert_eq!(expected, result);
    }

    #[test]
    fn basic_single_attribute() {
        let lines = [">> id: delta", ""].join("\n");
        let source = lines.as_str();
        let expected = Ok((
            "",
            Some(vec![SectionAttributeV2::Attribute {
                key: Some("id".to_string()),
                value: Some("delta".to_string()),
            }]),
        ));
        let result = section_attributes_v2(source);
        assert_eq!(expected, result);
    }

    #[test]
    fn basic_multiple_attributes() {
        let lines = [">> id: echo", ">> class: foxtrot hotel", ""].join("\n");
        let source = lines.as_str();
        let expected = Ok((
            "",
            Some(vec![
                SectionAttributeV2::Attribute {
                    key: Some("id".to_string()),
                    value: Some("echo".to_string()),
                },
                SectionAttributeV2::Attribute {
                    key: Some("class".to_string()),
                    value: Some("foxtrot hotel".to_string()),
                },
            ]),
        ));
        let result = section_attributes_v2(source);
        assert_eq!(expected, result);
    }

    #[test]
    fn single_attribute_with_just_a_key() {
        let lines = [">> sierra", ""].join("\n");
        let source = lines.as_str();
        let expected = Ok((
            "",
            Some(vec![SectionAttributeV2::Attribute {
                key: Some("sierra".to_string()),
                value: None,
            }]),
        ));
        let result = section_attributes_v2(source);
        assert_eq!(expected, result);
    }

    #[test]
    fn single_attribute_with_just_a_key_then_key_value() {
        let lines = [">> sierra", ">> class: tango", ""].join("\n");
        let source = lines.as_str();
        let expected = Ok((
            "",
            Some(vec![
                SectionAttributeV2::Attribute {
                    key: Some("sierra".to_string()),
                    value: None,
                },
                SectionAttributeV2::Attribute {
                    key: Some("class".to_string()),
                    value: Some("tango".to_string()),
                },
            ]),
        ));
        let result = section_attributes_v2(source);
        assert_eq!(expected, result);
    }
}

// Ok((
//     "",
//     Some(vec![SectionAttribute::Attribute {
//         key: Some("class".to_string()),
//         value: Some("highlighted".to_string()),
//     }]),
// ))

// let (a, b) = alt((tuple((take_until("\n"), tag("\n"))).map(|x| x.0), rest))(source)?;
// // The above line was added to properly pull in
// // secondary attributes. The bloew lines might have
// // some kruft that's unnecessary now. Not worth
// // removing unless it becomes a problem
// let (_, b) = alt((
//     tuple((
//         is_not(":"),
//         tag(":"),
//         space1,
//         not_line_ending,
//         alt((line_ending, eof)),
//     ))
//     .map(|x| (x.0, x.3)),
//     tuple((not_line_ending, alt((line_ending, eof)))).map(|x| (x.0, "")),
// ))(b)?;
// if b.1.is_empty() {
//     Ok((
//         a.trim(),
//         SectionAttribute::Attribute {
//             key: Some(b.0.to_string()),
//             value: None,
//         },
//     ))
// } else {
//     Ok((
//         a,
//         SectionAttribute::Attribute {
//             key: Some(b.0.to_string()),
//             value: Some(b.1.to_string()),
//         },
//     ))
// }
//

// #[test]
// fn test_section_attribute_only_key() {
//     let lines = ["rust", ""].join("\n");
//     let source = lines.as_str();
//     let expected = Ok((
//         "",
//         SectionAttribute::Attribute {
//             key: Some("rust".to_string()),
//             value: None,
//         },
//     ));
//     let result = section_attribute(source);
//     assert_eq!(expected, result);
// }

// #[test]
// fn test_section_attribute_only_key_no_following_line() {
//     // This is for dealing with items taht get parsed out
//     // like youtube that only have attributes.
//     let lines = ["apple"].join("\n");
//     let source = lines.as_str();
//     let expected = Ok((
//         "",
//         SectionAttribute::Attribute {
//             key: Some("apple".to_string()),
//             value: None,
//         },
//     ));
//     let result = section_attribute(source);
//     assert_eq!(expected, result);
// }

// #[test]
// fn single_first_item_followed_by_key_value() {
//     // This is for dealing with items taht get parsed out
//     // like youtube that only have attributes.
//     let lines = ["bravo", ">> charlie: delta"].join("\n");
//     let source = lines.as_str();
//     let expected = Ok((
//         ">> charlie: delta",
//         SectionAttribute::Attribute {
//             key: Some("bravo".to_string()),
//             value: None,
//         },
//     ));
//     let result = section_attribute(source);
//     assert_eq!(expected, result);
// }
