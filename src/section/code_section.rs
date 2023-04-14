use crate::block::block::*;
// use crate::block::block::block_of_code;
// use crate::block::block_of_code::*;
// use crate::block::block_of_code;
use crate::section::section::*;
use crate::section::attributes_for_section::*;
use nom::bytes::complete::tag;
// use nom::combinator::eof;
use nom::multi::many0;
//use nom::multi::many_till;
use nom::sequence::preceded;
use nom::IResult;

pub fn code_section(source: &str) -> IResult<&str, Section> {
    // dbg!(source);
    let (source, att_capture) = many0(preceded(tag(">> "), section_attribute))(source).unwrap();
    let attributes = if att_capture.is_empty() {
        None
    } else {
        Some(att_capture)
    };
    // let (source, b) = many_till(block_of_code, eof)(source.trim()).unwrap();
    // let children = if b.0.is_empty() { None } else { Some(b.0) };
    let children = if source.trim().is_empty() {
        None
    } else {
        Some(Block::CodeBlock {
            text: Some(source.trim().to_string()),
        })
    };

    Ok((
        source,
        Section::CodeSection {
            attributes,
            children,
        },
    ))

}
