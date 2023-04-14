use crate::section::section::*;
use crate::section::attributes_for_section::*;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;

pub fn attributes(source: &str) -> IResult<&str, Section> {
    dbg!(source);
    let (source, att_capture) = many0(preceded(tag(">> "), section_attribute))(source).unwrap();
    let attributes = if att_capture.is_empty() {
        None
    } else {
        Some(att_capture)
    };
    Ok((
        source,
        Section::AttributesSection {
            attributes,
            // Note, this is hard coded to None for my
            // implementation because I'm only using key
            // value pairs in the attributes. It's avaialbe
            // for usage though in the AST for other data
            // formats
            children: None,
        },
    ))
}

#[cfg(test)]
mod tests {
use crate::parse::parse;
use crate::section::section::*;
use crate::section::attributes_for_section::*;
use crate::wrapper::wrapper::*;

#[test]
fn basic_attributes() {
    let lines = vec!["-> attributes", ">> somekey: some value"].join("\n");
    let source = lines.as_str();
    let expected = Wrapper::Page {
        children: Some(vec![Section::AttributesSection {
            attributes: Some(vec![
                (SectionAttribute::Attribute {
                    key: Some("somekey".to_string()),
                    value: Some("some value".to_string()),
                }),
            ]),
            children: None,
        }]),
    };
    let result = parse(source).unwrap().1;
    assert_eq!(expected, result);
}
}