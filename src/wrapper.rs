use crate::section::section::section;
use crate::section::section::Section;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Wrapper {
    Page { children: Option<Vec<Section>> },
}

pub fn wrapper(source: &str) -> IResult<&str, Wrapper> {
    let (_, sections) = many_till(section, eof)(source)?;
    let response = Wrapper::Page {
        children: Some(sections.0),
    };
    Ok(("", response))
}
