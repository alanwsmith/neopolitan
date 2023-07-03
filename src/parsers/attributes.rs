#![allow(unused_variables)]
pub mod attribute;
pub mod global;

use crate::parsers::attributes::attribute::attribute;
use crate::parsers::attributes::attribute::Attribute;
use nom::multi::many0;
use nom::IResult;

pub fn attributes(
    source: &str,
) -> IResult<&str, Vec<Attribute>> {
    dbg!(&source);
    let (source, attrs) = many0(attribute)(source)?;
    Ok((source, attrs))
}
