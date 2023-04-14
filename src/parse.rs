use crate::wrapper::wrapper::Wrapper;
use crate::section::section::*;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub fn parse(source: &str) -> IResult<&str, Wrapper> {
    let (_, sections) = many_till(section, eof)(source)?;
    let expected = Wrapper::Page {
        children: Some(sections.0),
    };
    Ok(("", expected))
}
