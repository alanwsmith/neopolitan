use crate::blocks::Block;
use crate::section_attrs::SecAttr;
use crate::sections::h::h;
use crate::sections::title::title;
use nom::branch::alt;
use nom::multi::many0;
use nom::IResult;

pub mod h;
pub mod title;

#[derive(Debug, PartialEq)]
pub enum Section {
    H2 {
        attrs: Vec<SecAttr>,
        headline: Block,
        paragraphs: Vec<Block>,
    },
    Title {
        attrs: Vec<SecAttr>,
        headline: Block,
        paragraphs: Vec<Block>,
    },
    None,
}

pub fn sections(source: &str) -> IResult<&str, Vec<Section>> {
    let (source, results) = many0(alt((title, h)))(source)?;
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
    pub fn solo_basic_integration() {
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
            "",
            "-> h2",
            ">> id: victor",
            "",
            "Crack the walnut.",
            "Fasten <<two|strong>> pins.",
            "",
            "",
            "",
            "<<Guess the|abbr>> results.",
            "Hoist it up.",
        ]
        .join("\n");
        let expected = vec![
            Section::Title {
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
            },
            Section::H2 {
                attrs: vec![SecAttr::Id("victor".to_string())],
                headline: Block::Headline {
                    snippets: vec![
                        Tag::Text {
                            text: "Crack the walnut. Fasten ".to_string(),
                        },
                        Tag::Strong {
                            attrs: vec![],
                            text: "two".to_string(),
                        },
                        Tag::Text {
                            text: " pins.".to_string(),
                        },
                    ],
                },
                paragraphs: vec![Block::Paragraph {
                    snippets: vec![
                        Tag::Abbr {
                            attrs: vec![],
                            text: "Guess the".to_string(),
                        },
                        Tag::Text {
                            text: " results. Hoist it up.".to_string(),
                        },
                    ],
                }],
            },
        ];
        assert_eq!(expected, sections(lines.as_str()).unwrap().1);
    }
}
