use crate::snippets::strong::strong;
use crate::snippets::text::text;
use crate::tag_attr::TagAttr;
use nom::branch::alt;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub mod strong;
pub mod text;

#[derive(Debug, PartialEq)]
pub enum Snippet {
    Text { text: String },
    Strong { attrs: Vec<TagAttr>, text: String },
}

pub fn snippets(source: &str) -> IResult<&str, Vec<Snippet>> {
    let (source, snippets) = many_till(alt((strong, text)), eof)(source)?;
    Ok((source, snippets.0))
}
