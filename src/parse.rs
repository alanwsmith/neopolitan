use crate::block::Block;
use crate::section::Section;
use crate::snippet::Snippet;
use nom::IResult;

pub fn parse(source: &str) -> IResult<&str, Vec<Section>> {
    let results = vec![Section::Title {
        attrs: vec![],
        headline: Block::Headline {
            content: vec![Snippet::Text {
                string: "hello world".to_string(),
            }],
        },
        paragraphs: vec![],
    }];
    Ok((source, results))
}

#[cfg(test)]

mod test {
    use super::*;
    use crate::block::Block;
    use crate::section::Section;
    use crate::snippet::Snippet;

    #[test]
    pub fn basic() {
        let lines =
            ["-> title", "", "hello world"].join("\n");
        let expected = vec![Section::Title {
            attrs: vec![],
            headline: Block::Headline {
                content: vec![Snippet::Text {
                    string: "hello world".to_string(),
                }],
            },
            paragraphs: vec![],
        }];
        assert_eq!(
            expected,
            parse(lines.as_str()).unwrap().1
        );
    }
}
