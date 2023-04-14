use crate::section::section::*;
use crate::section::section_attributes::*;
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
