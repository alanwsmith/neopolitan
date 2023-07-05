use crate::tags::Tag;
use nom::character::complete::not_line_ending;
use nom::IResult;

pub fn text(source: &str) -> IResult<&str, Tag> {
    let (source, text_string) = not_line_ending(source)?;
    Ok((
        source,
        Tag::Text {
            text: text_string.to_string(),
        },
    ))
}
