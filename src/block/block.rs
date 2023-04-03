use crate::content::content::Content;
use nom::IResult;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Block {
    ListItem {
        attributes: Option<HashMap<String, String>>,
        children: Option<Vec<Block>>,
    },
    P {
        attributes: Option<HashMap<String, String>>,
        children: Option<Vec<Content>>,
    },
    Placeholder,
}

pub fn block(_source: &str) -> IResult<&str, Block> {
    // let (source, block) = alt((
    // ))(source)?

    Ok((
        "",
        Block::P {
            attributes: None,
            children: Some(vec![Content::Text {
                text: Some("the quick brown fox".to_string()),
            }]),
        },
    ))
}

// pub fn title(source: &str) -> IResult<&str, Section> {
//     let (source, _) = space0(source);
//     Ok((
//         "",
//         Section::Title {
//             attributes: None,
//             children: Some(vec![Block::P {
//                 attributes: None,
//                 children: Some(vec![Content::Text {
//                     text: Some(source.trim().to_string()),
//                 }]),
//             }]),
//         },
//     ))
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_block() {
        let source = "the quick brown fox";
        let expected = Ok((
            "",
            Block::P {
                attributes: None,
                children: Some(vec![Content::Text {
                    text: Some("the quick brown fox".to_string()),
                }]),
            },
        ));
        let result = block(source);
        assert_eq!(expected, result);
    }
}
