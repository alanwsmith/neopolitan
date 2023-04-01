use crate::chunk::Chunk;
use crate::split::split;
use nom::IResult;

pub fn strong<'a>(
    source: &'a str,
    _raw_attributes: &'a str,
    remainder: &'a str,
) -> IResult<&'a str, Chunk> {
    let (_, stuff) = split(source, "|")?;
    let response = Chunk::Strong {
        attributes: None,
        value: Some(stuff[0].to_string()),
    };
    Ok((remainder, response))
}
