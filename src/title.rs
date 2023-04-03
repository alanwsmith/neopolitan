use crate::block::Block;
use crate::content::Content;
use crate::p::p;
use crate::section::Section;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub fn title(source: &str) -> IResult<&str, Section> {
    let (source, _) = space0(source)?;
    // let (source, paragraphs) = many_till(title_paragraph, eof)(source)?;
    Ok((
        "",
        Section::Title {
            attributes: None,
            children: Some(vec![Block::P {
                attributes: None,
                children: Some(vec![Content::Text {
                    text: Some(source.trim().to_string()),
                }]),
            }]),
        },
    ))
}

pub fn title_dev(source: &str) -> IResult<&str, Section> {
    let (source, _) = space0(source)?;
    let (_source, _paragraphs) = many_till(title_paragraph, eof)(source)?;
    Ok((
        "",
        Section::Title {
            attributes: None,
            children: Some(vec![Block::P {
                attributes: None,
                children: Some(vec![Content::Text {
                    text: Some("Hello, World".to_string()),
                }]),
            }]),
        },
    ))
}

fn title_paragraph(source: &str) -> IResult<&str, &str> {
    let (_, b) = p(source)?;
    dbg!(&b);
    Ok(("", ""))
}

// #[test]
// fn dev_title_test() {
//     let source = "\nHello, World";
//     let expected = Ok((
//         "",
//         Section::Title {
//             attributes: None,
//             children: Some(vec![Block::P {
//                 attributes: None,
//                 children: Some(vec![Content::Text {
//                     text: Some("Hello, World".to_string()),
//                 }]),
//             }]),
//         },
//     ));
//     let result = title_dev(source);
//     assert_eq!(expected, result);
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::section::Section;

    //

    #[test]
    fn basic_title_response() {
        let source = "\nHello, World";
        let expected = Ok((
            "",
            Section::Title {
                attributes: None,
                children: Some(vec![Block::P {
                    attributes: None,
                    children: Some(vec![Content::Text {
                        text: Some("Hello, World".to_string()),
                    }]),
                }]),
            },
        ));
        let result = title(source);
        assert_eq!(expected, result);
    }

    //
}
