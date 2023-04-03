#![allow(warnings)]
use neopolitan::block::Block;
use neopolitan::content::Content;
use neopolitan::section::*;
use neopolitan::title::title;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Wrapper {
    Page { children: Option<Vec<Section>> },
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
