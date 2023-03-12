use crate::parse::Content;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::IResult;

pub fn get_text(source: &str) -> IResult<&str, Content> {
    let (source, _) = multispace0(source)?;
    let (source, content) = not_line_ending(source)?;
    Ok((
        source,
        Content::PlainText {
            value: content.trim().to_string(),
        },
    ))
}
