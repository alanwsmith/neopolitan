use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::combinator::opt;
use nom::multi::many0;
use nom::multi::separated_list0;
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
