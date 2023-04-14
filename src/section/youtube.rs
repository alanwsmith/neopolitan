use crate::section::section::*;
use crate::section::section_attributes::*;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;

pub fn youtube(source: &str) -> IResult<&str, Section> {
    let (source, att_capture) = many0(preceded(tag(">> "), section_attribute))(source).unwrap();
    let attributes = if att_capture.is_empty() {
        None
    } else {
        Some(att_capture)
    };
    Ok((source, Section::YouTubeSection { attributes }))
}
