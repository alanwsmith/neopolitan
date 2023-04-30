use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::combinator::opt;
use nom::multi::many0;
use nom::multi::separated_list0;
use nom::IResult;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum SectionAttribute {
    Attribute {
        key: Option<String>,
        value: Option<String>,
    },
}

pub fn section_attributes(source: &str) -> IResult<&str, Option<Vec<SectionAttribute>>> {
    let (remainder, b) = many0(section_attribute)(source)?;
    if b.is_empty() {
        Ok((remainder, None))
    } else {
        Ok((remainder, Some(b)))
    }
}

pub fn section_attribute(source: &str) -> IResult<&str, SectionAttribute> {
    let (remainder, _) = tag(">>")(source)?;
    let (remainder, _) = multispace1(remainder)?;
    let (remainder, captured) = not_line_ending(remainder)?;
    let (remainder, _) = line_ending(remainder)?;
    let (_, parts) = split(captured, ":")?;
    if parts.len() == 1 {
        let response = SectionAttribute::Attribute {
            key: Some(parts[0].trim().to_string()),
            value: None,
        };
        Ok((remainder, response))
    } else {
        // deal with values that have colons in them
        let mut new_vec: Vec<String> = vec![];
        parts
            .iter()
            .skip(1)
            .for_each(|x| new_vec.push(x.trim().to_string()));
        let response = SectionAttribute::Attribute {
            key: Some(parts[0].trim().to_string()),
            // value: Some(parts[1].trim().to_string()),
            value: Some(new_vec.join(":")),
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
    use crate::section::section_attributes::SectionAttribute;
    use crate::section::section_attributes::*;
    #[test]
    fn no_attributes() {
        let lines = ["Twist the valve"].join("\n");
        let source = lines.as_str();
        let expected = Ok(("Twist the valve", None));
        let result = section_attributes(source);
        assert_eq!(expected, result);
    }

    #[test]
    fn basic_single_attribute() {
        let lines = [">> id: delta", ""].join("\n");
        let source = lines.as_str();
        let expected = Ok((
            "",
            Some(vec![SectionAttribute::Attribute {
                key: Some("id".to_string()),
                value: Some("delta".to_string()),
            }]),
        ));
        let result = section_attributes(source);
        assert_eq!(expected, result);
    }

    #[test]
    fn basic_multiple_attributes() {
        let lines = [">> id: echo", ">> class: foxtrot hotel", ""].join("\n");
        let source = lines.as_str();
        let expected = Ok((
            "",
            Some(vec![
                SectionAttribute::Attribute {
                    key: Some("id".to_string()),
                    value: Some("echo".to_string()),
                },
                SectionAttribute::Attribute {
                    key: Some("class".to_string()),
                    value: Some("foxtrot hotel".to_string()),
                },
            ]),
        ));
        let result = section_attributes(source);
        assert_eq!(expected, result);
    }

    #[test]
    fn single_attribute_with_just_a_key() {
        let lines = [">> sierra", ""].join("\n");
        let source = lines.as_str();
        let expected = Ok((
            "",
            Some(vec![SectionAttribute::Attribute {
                key: Some("sierra".to_string()),
                value: None,
            }]),
        ));
        let result = section_attributes(source);
        assert_eq!(expected, result);
    }

    #[test]
    fn single_attribute_with_just_a_key_then_key_value() {
        let lines = [">> sierra", ">> class: tango", ""].join("\n");
        let source = lines.as_str();
        let expected = Ok((
            "",
            Some(vec![
                SectionAttribute::Attribute {
                    key: Some("sierra".to_string()),
                    value: None,
                },
                SectionAttribute::Attribute {
                    key: Some("class".to_string()),
                    value: Some("tango".to_string()),
                },
            ]),
        ));
        let result = section_attributes(source);
        assert_eq!(expected, result);
    }

    #[test]
    fn with_colon_in_the_value() {
        let lines = [">> src: https://www.example.com/", ""].join("\n");
        let source = lines.as_str();
        let expected = Ok((
            "",
            Some(vec![SectionAttribute::Attribute {
                key: Some("src".to_string()),
                value: Some("https://www.example.com/".to_string()),
            }]),
        ));
        let result = section_attributes(source);
        assert_eq!(expected, result);
    }
}
