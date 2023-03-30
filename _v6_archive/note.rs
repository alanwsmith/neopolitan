use crate::attributes::attributes;
use crate::chunk::Chunk;
use crate::section::*;
use crate::text::*;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::IResult;

pub fn note(source: &str) -> IResult<&str, Section> {
    let (source, attributes) = attributes(source)?;
    let (_, paragraphs) = split_parts(source, "\n\n")?;
    let children: Vec<Chunk> = paragraphs
        .iter()
        .map(|p| Chunk::P {
            attributes: None,
            children: text(p).unwrap().1,
        })
        .collect();
    let expected = Section::NoteSection {
        attributes,
        children: Some(children),
    };
    Ok(("", expected))
}

fn split_parts<'a>(source: &'a str, separator: &'a str) -> IResult<&'a str, Vec<&'a str>> {
    let (remainder, content) = opt(tag(separator))(source)?;
    match content {
        None => {
            let (_, items) = separated_list0(tag(separator), is_not(separator))(remainder)?;
            Ok(("", items))
        }
        Some(..) => {
            let (_, items) = separated_list0(tag(separator), is_not(separator))(remainder)?;
            Ok(("", items))
        }
    }
}
