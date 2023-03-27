use crate::chunk::Chunk;
use crate::parse_text_attributes::parse_text_attributes;
use crate::split::split;
use nom::IResult;

pub fn link<'a>(source: &'a str, remainder: &'a str) -> IResult<&'a str, Chunk> {
    let (_, stuff) = split(source, "|")?;
    if stuff.len() > 2 {
    let (_, attributes) = parse_text_attributes(stuff[2])?;
    let response = Chunk::Link {
        attributes,
        value: Some(stuff[1].to_string()),
        url: Some(stuff[0].to_string()),
    };
    Ok((remainder, response))
    } else {
    let response = Chunk::Link {
        attributes: None,
        value: Some(stuff[1].to_string()),
        url: Some(stuff[0].to_string()),
    };
    Ok((remainder, response))
    } 
}
