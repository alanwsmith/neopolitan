use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::character::complete::space1;
use nom::combinator::opt;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum SectionAttribute {
    Attribute {
        key: Option<String>,
        value: Option<String>,
    },
}

pub fn section_attribute(source: &str) -> IResult<&str, SectionAttribute> {
    let (a, b) = opt(tuple((
        is_not(":"),
        tag(":"),
        space1,
        not_line_ending,
        line_ending,
    )))(source)?;
    Ok((
        a,
        SectionAttribute::Attribute {
            key: Some(b.unwrap().0.to_string()),
            value: Some(b.unwrap().3.to_string()),
        },
    ))
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
