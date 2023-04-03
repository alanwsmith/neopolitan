use crate::block::Block;
use crate::content::Content;
use crate::p::p;
use crate::section::Section;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub fn title(source: &str) -> IResult<&str, Section> {
    let (source, _) = multispace0(source)?;
    let (_source, paragraphs) = many_till(p, eof)(source)?;
    Ok((
        "",
        Section::Title {
            attributes: None,
            children: Some(paragraphs.0),
        },
    ))
}

#[test]
fn dev_title_test() {
    let source = "\nHello, World\n\nquick brown fox";
    let expected = Ok((
        "",
        Section::Title {
            attributes: None,
            children: Some(vec![
                Block::P {
                    attributes: None,
                    children: Some(vec![Content::Text {
                        text: Some("Hello, World".to_string()),
                    }]),
                },
                Block::P {
                    attributes: None,
                    children: Some(vec![Content::Text {
                        text: Some("quick brown fox".to_string()),
                    }]),
                },
            ]),
        },
    ));
    let result = title(source);
    assert_eq!(expected, result);
}

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
