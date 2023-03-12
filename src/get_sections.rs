use crate::parse::Section;
use crate::section::*;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub fn get_sections(source: &str) -> IResult<&str, Vec<Section>> {
    let (source, sections) = many_till(section, eof)(source)?;
    Ok((source, sections.0))
}
