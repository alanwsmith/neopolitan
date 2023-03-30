use crate::chunk::Chunk;
use crate::inline_language::inline_language;
use crate::parse_text_attributes::parse_text_attributes;
use crate::split::split;
use nom::IResult;

pub fn code_inline<'a>(
    source: &'a str,
    raw_attributes: &'a str,
    remainder: &'a str,
) -> IResult<&'a str, Chunk> {
    let (_, stuff) = split(source, "|")?;
    let (_, attributes) = parse_text_attributes(raw_attributes)?;
    let (_, language) = inline_language(raw_attributes)?;
    let response = Chunk::InlineCode{
        language,
        attributes,
        value: Some(stuff[0].to_string()),
    };
    Ok((remainder, response))
}
