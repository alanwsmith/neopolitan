use crate::section::section::*;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub fn parse(source: &str) -> IResult<&str, Option<Vec<Section>>> {
    let (_, sections) = many_till(section, eof)(source)?;
    Ok(("", Some(sections.0)))
}
