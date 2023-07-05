use crate::snippets::Snippet;
use nom::character::complete::not_line_ending;
use nom::IResult;

pub fn text(source: &str) -> IResult<&str, Snippet> {
    let (source, text_string) = not_line_ending(source)?;
    Ok((
        source,
        Snippet::Text {
            text: text_string.to_string(),
        },
    ))
}
