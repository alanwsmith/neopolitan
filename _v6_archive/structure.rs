use crate::section::*;
use crate::wrapper::*;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub fn structure(source: &str) -> IResult<&str, Option<Wrapper>> {
    let (_, sections) = many_till(section, eof)(source).unwrap();
    let p = Some(Wrapper::Post {
        attributes: None,
        children: Some(sections.0),
    });
    Ok(("", p))
}
