
// This is where all the global and specific
// attributes are defined for the enum


#[derive(Debug, PartialEq)]
pub enum Attribute {
    Class(Vec<String>),
    None
}


// pub mod attribute;
// pub mod global;

// use crate::parsers::attribute::attribute::attribute;
// use crate::parsers::attribute::attribute::Attribute;
// use nom::multi::many0;
// use nom::IResult;

// pub fn attributes(
//     source: &str,
// ) -> IResult<&str, Vec<Attribute>> {
//     dbg!(&source);
//     let (source, attrs) = many0(attribute)(source)?;
//     Ok((source, attrs))
// }
