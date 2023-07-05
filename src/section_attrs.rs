use crate::section_attrs::class::class;
use crate::section_attrs::id::id;
use nom::multi::many0;
use nom::IResult;
use nom::branch::alt;

pub mod class;
pub mod id;

#[derive(Debug, PartialEq)]
pub enum SecAttr {
    Class(Vec<String>),
    Id(String),
}

pub fn sec_attrs(source: &str) -> IResult<&str, Vec<SecAttr>> {
    let (source, attrs) = many0(alt((class, id)))(source.trim())?;
    Ok((source, attrs))
}
