use crate::snippets::Snippet;
use nom::IResult;

pub fn text(source: &str) -> IResult<&str, Snippet> {
    Ok((
        source,
        Snippet::Text {
            text: "".to_string(),
        },
    ))
}
