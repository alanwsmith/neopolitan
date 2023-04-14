// use crate::block::block::*;
use crate::section::section::*;
use crate::section::attributes_for_section::*;
use nom::bytes::complete::tag;
// use nom::combinator::eof;
use nom::multi::many0;
// use nom::multi::many_till;
use nom::sequence::preceded;
use nom::IResult;

pub fn vimeo(source: &str) -> IResult<&str, Section> {
    let (source, att_capture) = many0(preceded(tag(">> "), section_attribute))(source).unwrap();
    let attributes = if att_capture.is_empty() {
        None
    } else {
        Some(att_capture)
    };
    Ok((source, Section::VimeoSection { attributes }))
}
