use crate::tag_attrs::TagAttr;
use crate::tags::less_than::less_than;
use crate::tags::strong::strong;
use crate::tags::text::text;
use nom::branch::alt;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub mod less_than;
pub mod strong;
pub mod text;

#[derive(Debug, PartialEq)]
pub enum Tag {
    LessThan { text: String },
    Text { text: String },
    Strong { attrs: Vec<TagAttr>, text: String },
}

pub fn tags(source: &str) -> IResult<&str, Vec<Tag>> {
    dbg!(&source);
    let (source, snippets) =
        many_till(alt((less_than, strong, text)), eof)(source)?;
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
