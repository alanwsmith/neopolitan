use crate::blocks::Block;
use crate::section_attrs::SecAttr;
use crate::sections::title::title;
use nom::multi::many0;
use nom::IResult;

pub mod title;

#[derive(Debug, PartialEq)]
pub enum Section {
    Title {
        attrs: Vec<SecAttr>,
        headline: Block,
        paragraphs: Vec<Block>,
    },
    None,
}

pub fn sections(source: &str) -> IResult<&str, Vec<Section>> {
    let (source, results) = many0(title)(source)?;
    Ok((source, results))
}

#[cfg(test)]

mod test {
    use super::*;
    use crate::blocks::Block;
    use crate::section_attrs::SecAttr;
    use crate::sections::Section;
    use crate::tags::Tag;

    #[test]
    pub fn basic() {
        let lines = [
            "-> title",
            ">> class: alfa",
            "",
            "bravo charlie",
            "delta echo",
            "",
            "foxtrot <golf",
            "hotel",
            "",
            "whiskey <<tango|strong>> sierra",
        ]
        .join("\n");
        let expected = vec![Section::Title {
            attrs: vec![SecAttr::Class(vec!["alfa".to_string()])],

            headline: Block::Headline {
                snippets: vec![Tag::Text {
                    text: "bravo charlie delta echo".to_string(),
                }],
            },
            paragraphs: vec![
                Block::Paragraph {
                    snippets: vec![
                        Tag::Text {
                            text: "foxtrot ".to_string(),
                        },
                        Tag::LessThan {
                            text: "<g".to_string(),
                        },
                        Tag::Text {
                            text: "olf hotel".to_string(),
                        },
                    ],
                },
                Block::Paragraph {
                    snippets: vec![
                        Tag::Text {
                            text: "whiskey ".to_string(),
                        },
                        Tag::Strong {
                            attrs: vec![],
                            text: "tango".to_string(),
                        },
                        Tag::Text {
                            text: " sierra".to_string(),
                        },
                    ],
                },
            ],
        }];
        assert_eq!(expected, sections(lines.as_str()).unwrap().1);
    }
}
