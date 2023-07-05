use crate::tag_attrs::TagAttr;
use crate::tags::abbr::abbr;
use crate::tags::em::em;
use crate::tags::less_than::less_than;
use crate::tags::q::q;
use crate::tags::s::s;
use crate::tags::span::span;
use crate::tags::strong::strong;
use crate::tags::sub::sub;
use crate::tags::sup::sup;
use crate::tags::text::text;
use nom::branch::alt;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub mod abbr;
pub mod basic;
pub mod em;
pub mod less_than;
pub mod q;
pub mod s;
pub mod span;
pub mod strong;
pub mod sub;
pub mod sup;
pub mod text;

#[derive(Debug, PartialEq)]
pub enum Tag {
    Abbr { attrs: Vec<TagAttr>, text: String },
    Em { attrs: Vec<TagAttr>, text: String },
    LessThan { text: String },
    Text { text: String },
    Q { attrs: Vec<TagAttr>, text: String },
    S { attrs: Vec<TagAttr>, text: String },
    Span { attrs: Vec<TagAttr>, text: String },
    Strong { attrs: Vec<TagAttr>, text: String },
    Sub { attrs: Vec<TagAttr>, text: String },
    Sup { attrs: Vec<TagAttr>, text: String },
}

pub fn tags(source: &str) -> IResult<&str, Vec<Tag>> {
    dbg!(&source);
    let (source, snippets) = many_till(
        alt((less_than, abbr, em, strong, text, q, s, span, sub, sup)),
        eof,
    )(source)?;
    Ok((source, snippets.0))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn basic_text() {
        let line = "the quick brown fox";
        let expected = vec![Tag::Text {
            text: "the quick brown fox".to_string(),
        }];
        assert_eq!(expected, tags(line).unwrap().1);
    }

    #[test]
    pub fn text_with_strong() {
        let line = "alfa <<bravo|strong>> charlie";
        let expected = vec![
            Tag::Text {
                text: "alfa ".to_string(),
            },
            Tag::Strong {
                attrs: vec![],
                text: "bravo".to_string(),
            },
            Tag::Text {
                text: " charlie".to_string(),
            },
        ];
        assert_eq!(expected, tags(line).unwrap().1);
    }

    #[test]
    pub fn less_than() {
        let line = "delta <- <<echo|strong>> foxtrot";
        let expected = vec![
            Tag::Text {
                text: "delta ".to_string(),
            },
            Tag::LessThan {
                text: "<-".to_string(),
            },
            Tag::Text {
                text: " ".to_string(),
            },
            Tag::Strong {
                attrs: vec![],
                text: "echo".to_string(),
            },
            Tag::Text {
                text: " foxtrot".to_string(),
            },
        ];
        assert_eq!(expected, tags(line).unwrap().1);
    }
}
