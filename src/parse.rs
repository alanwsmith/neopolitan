use crate::section::title::title;
use crate::section::Section;
use nom::multi::many0;
use nom::IResult;

pub fn parse(source: &str) -> IResult<&str, Vec<Section>> {
    let (source, results) = many0(title)(source)?;
    Ok((source, results))
}

#[cfg(test)]

mod test {
    use super::*;
    use crate::block::Block;
    use crate::sec_attr::SecAttr;
    use crate::section::Section;
    use crate::snippet::Snippet;

    #[test]
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
            "whiskey tango",
        ]
        .join("\n");
        let expected = vec![Section::Title {
            attrs: vec![SecAttr::Class(vec![
                "alfa".to_string()
            ])],
            headline: Block::Headline {
                content: vec![Snippet::Text {
                    string: "bravo charlie delta echo"
                        .to_string(),
                }],
            },
            paragraphs: vec![
                Block::Paragraph {
                    content: vec![Snippet::Text {
                        string: "foxtrot golf hotel"
                            .to_string(),
                    }],
                },
                Block::Paragraph {
                    content: vec![Snippet::Text {
                        string: "whiskey tango".to_string(),
                    }],
                },
            ],
        }];
        assert_eq!(
            expected,
            parse(lines.as_str()).unwrap().1
        );
    }
}
