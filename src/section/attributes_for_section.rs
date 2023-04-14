use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum SectionAttribute {
    Attribute {
        key: Option<String>,
        value: Option<String>,
    },
}

#[test]
fn test_section_attribute() {
    let lines = ["class: highlighted", ""].join("\n");
    let source = lines.as_str();
    let expected = Ok((
        "",
        SectionAttribute::Attribute {
            key: Some("class".to_string()),
            value: Some("highlighted".to_string()),
        },
    ));
    let result = section_attribute(source);
    assert_eq!(expected, result);
}

#[test]
fn test_section_attribute_only_key() {
    let lines = ["rust", ""].join("\n");
    let source = lines.as_str();
    let expected = Ok((
        "",
        SectionAttribute::Attribute {
            key: Some("rust".to_string()),
            value: None,
        },
    ));
    let result = section_attribute(source);
    assert_eq!(expected, result);
}

#[test]
fn test_section_attribute_only_key_no_following_line() {
    // This is for dealing with items taht get parsed out
    // like youtube that only have attributes.
    let lines = ["apple"].join("\n");
    let source = lines.as_str();
    let expected = Ok((
        "",
        SectionAttribute::Attribute {
            key: Some("apple".to_string()),
            value: None,
        },
    ));
    let result = section_attribute(source);
    assert_eq!(expected, result);
}

pub fn section_attribute(source: &str) -> IResult<&str, SectionAttribute> {
    let (a, b) = alt((
        tuple((
            is_not(":"),
            tag(":"),
            space1,
            not_line_ending,
            alt((line_ending, eof)),
        ))
        .map(|x| (x.0, x.3)),
        tuple((not_line_ending, alt((line_ending, eof)))).map(|x| (x.0, "")),
    ))(source)?;
    // dbg!(&b);
    if b.1.is_empty() {
        Ok((
            a,
            SectionAttribute::Attribute {
                key: Some(b.0.to_string()),
                value: None,
            },
        ))
    } else {
        Ok((
            a,
            SectionAttribute::Attribute {
                key: Some(b.0.to_string()),
                value: Some(b.1.to_string()),
            },
        ))
    }
}
