// use crate::block::block::*;
use crate::block::unordered_list_item::*;
// use crate::content::content::*;
use crate::section::section::*;
use crate::section::section_attributes::*;
use nom::bytes::complete::tag;
use nom::combinator::eof;
use nom::multi::many0;
use nom::multi::many_till;
use nom::sequence::preceded;
use nom::IResult;

pub fn list(source: &str) -> IResult<&str, Section> {
    // dbg!(&source);
    let (source, att_capture) = many0(preceded(tag(">> "), section_attribute))(source).unwrap();
    let _attributes = if att_capture.is_empty() {
        None
    } else {
        Some(att_capture)
    };
    // let (source, b) = many_till(block, eof)(source.trim()).unwrap();
    let (source, b) = many_till(unordered_list_item, eof)(source.trim()).unwrap();
    let children = if b.0.is_empty() { None } else { Some(b.0) };
    // dbg!(&source);
    // dbg!(&children);
    Ok((
        source,
        Section::List {
            attributes: None,
            children,
        },
    ))
}
