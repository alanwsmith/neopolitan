use crate::sections::title::title;
use crate::sections::Section;
use nom::multi::many0;
use nom::IResult;


// TODO: Move this into sections.rs and 
// consolidate there


pub fn parse(source: &str) -> IResult<&str, Vec<Section>> {
    let (source, results) = many0(title)(source)?;
    Ok((source, results))
}

#[cfg(test)]

mod test {
    use super::*;
    use crate::blocks::Block;
    use crate::section_attrs::SecAttr;
    use crate::sections::Section;
    use crate::tags::Snippet;

    #[test]
    #[ignore]
    pub fn basic() {
        let lines = [
            "-> title",
            ">> class: alfa",
            "",
            "bravo charlie",
            "delta echo",
            "",
            "foxtrot golf",
            "hotel",
            "",
            "whiskey <<tango|strong>> sierra",
        ]
        .join("\n");
        let expected = vec![Section::Title {
            attrs: vec![SecAttr::Class(vec!["alfa".to_string()])],

            headline: Block::Headline {
                snippets: vec![Snippet::Text {
                    text: "bravo charlie delta echo".to_string(),
                }],
            },
            paragraphs: vec![
                Block::Paragraph {
                    snippets: vec![Snippet::Text {
                        text: "foxtrot golf hotel".to_string(),
                    }],
                },
                Block::Paragraph {
                    snippets: vec![
                        Snippet::Text {
                            text: "whiskey ".to_string(),
                        },
                        Snippet::Strong {
                            attrs: vec![],
                            text: "tango".to_string(),
                        },
                        Snippet::Text {
                            text: " sierra".to_string(),
                        },
                    ],
                },
            ],
        }];
        assert_eq!(expected, parse(lines.as_str()).unwrap().1);
    }
}
