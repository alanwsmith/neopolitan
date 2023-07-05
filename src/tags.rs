// use crate::snippets::strong::strong;
use crate::tags::text::text;
use crate::tag_attrs::TagAttr;
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
    let (source, snippets) = many_till(alt((text, text)), eof)(source)?;
    Ok((source, snippets.0))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn basic_text() {
        let line = "the quick brown fox";
        let expected = vec![Snippet::Text {
            text: "the quick brown fox".to_string(),
        }];
        assert_eq!(expected, snippets(line).unwrap().1);
    }
}
