use crate::enums::Wrapper;
use crate::wrapper::wrapper::*;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub fn parse(source: &str) -> IResult<&str, Wrapper> {
    dbg!(source);
    let (_, sections) = many_till(section, eof)(source)?;
    let expected = Wrapper::Page {
        children: sections.0,
    };
    Ok(("", expected))
}
