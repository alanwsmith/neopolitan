use crate::tag_attrs::TagAttr;
use crate::tags::strong::strong;
use crate::tags::text::text;
use nom::branch::alt;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub mod strong;
pub mod text;

#[derive(Debug, PartialEq)]
pub enum Tag {
    Text { text: String },
    Strong { attrs: Vec<TagAttr>, text: String },
}

pub fn tags(source: &str) -> IResult<&str, Vec<Tag>> {
    dbg!(&source);
    let (source, snippets) = many_till(alt((strong, text)), eof)(source)?;
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
    pub fn solo_text_with_strong() {
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
}
