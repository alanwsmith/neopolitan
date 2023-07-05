use crate::snippets::Snippet;
use nom::IResult;

pub fn strong(source: &str) -> IResult<&str, Snippet> {
    Ok((source, Snippet::Strong{ text: "".to_string(), attrs: vec![]}))
}