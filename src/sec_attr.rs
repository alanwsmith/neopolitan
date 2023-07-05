use crate::sec_attr::class::class;
use nom::multi::many0;
use nom::IResult;

pub mod class;

#[derive(Debug, PartialEq)]
pub enum SecAttr {
    Class(Vec<String>),
}

pub fn sec_attrs(
    source: &str,
) -> IResult<&str, Vec<SecAttr>> {
    let (source, attrs) = many0(class)(source.trim())?;
    Ok((source, attrs))
}
