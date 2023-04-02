#![allow(warnings)]
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Wrapper {
    Page { children: Option<Vec<Section>> },
}

#[derive(Debug, PartialEq)]
enum Section {
    Title {
        attributes: Option<HashMap<String, String>>,
        children: Option<Vec<Block>>,
    },
}

#[derive(Debug, PartialEq)]
enum Block {
    P {
        attributes: Option<HashMap<String, String>>,
        children: Option<Vec<Content>>,
    },
}

#[derive(Debug, PartialEq)]
enum Content {
    Text { value: Option<String> },
}

fn main() {
    let _string = "-> title\n\nHere it is";
}

fn parse(source: &str) -> IResult<&str, Wrapper> {
    let (a, b) = many_till(section, eof)(source)?;
    let page = Wrapper::Page {
        children: Some(vec![Section::Title {
            attributes: None,
            children: Some(vec![Block::P {
                attributes: None,
                children: Some(vec![Content::Text {
                    value: Some("Here it is".to_string()),
                }]),
            }]),
        }]),
    };
    Ok(("", page))
}

fn section(source: &str) -> IResult<&str, &str> {
    Ok(("", ""))
}

#[test]
fn test1() {
    let source = "-> title\n\nHere it is";
    let expected = Wrapper::Page {
        children: Some(vec![Section::Title {
            attributes: None,
            children: Some(vec![Block::P {
                attributes: None,
                children: Some(vec![Content::Text {
                    value: Some("Here it is".to_string()),
                }]),
            }]),
        }]),
    };
    let (_, result) = parse(source).unwrap();
    assert_eq!(expected, result);
}
