#![allow(warnings)]
use neopolitan::block::*;
use neopolitan::content::*;
use neopolitan::section::*;
use neopolitan::title::title;
use neopolitan::wrapper::*;
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

fn main() {
    let _string = "-> title\n\nHere it is";
}

// fn parse(source: &str) -> IResult<&str, Wrapper> {
//     let (a, b) = many_till(section, eof)(source)?;
//     let page = Wrapper::Page {
//         children: Some(vec![Section::Title {
//             attributes: None,
//             children: Some(vec![Block::P {
//                 attributes: None,
//                 children: Some(vec![Content::Text {
//                     value: Some("Here it is".to_string()),
//                 }]),
//             }]),
//         }]),
//     };
//     Ok(("", page))
// }

// Page
// vecSection
//    vecBlock
//       vecContent
//          Text
//          Link
//          etc...
