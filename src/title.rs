use crate::p::p;
use crate::section::Section;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub fn title(source: &str) -> IResult<&str, Section> {
    let (source, _) = multispace0(source)?;
    let (_source, paragraphs) = many_till(p, eof)(source)?;
    Ok((
        "",
        Section::Title {
            attributes: None,
            children: Some(paragraphs.0),
        },
    ))
}
