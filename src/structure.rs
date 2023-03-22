#![allow(warnings)]
// use crate::spec::Spec;
use crate::block::*;
use crate::content::*;
use crate::section::*;
use crate::wrapper::*;
// use crate::xob::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::multi::many0;
use nom::multi::many1;
use nom::multi::many_till;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::Err;
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;

pub fn structure(source: &str) -> IResult<&str, Option<Wrapper>> {
    let response = Some(Wrapper::Page {
        children: Some(vec![Section::Title {
            attributes: None,
            children: Some(vec![Block::H1 {
                attributes: None,
                children: Some(vec![Content::Text {
                    attributes: None,
                    value: Some("Welcome To Neopolitan".to_string()),
                }]),
            }]),
        }]),
    });

    Ok(("", response))
}

// pub fn structure(source: &str) -> IResult<&str, Option<Xob>> {
//     let response = Some(Wrapper {
//         spec: Wrapper::Page,
//         attributes: None,
//         children: Some(vec![Section {
//             spec: Section::Title,
//             attributes: None,
//             extras: None,
//             children: Some(vec![Chunk {
//                 spec: Chunk::H1,
//                 attributes: None,
//                 children: Some(vec![Content {
//                     spec: Content::Text,
//                     attributes: None,
//                     children: None,
//                 }]),
//             }]),
//         }]),
//     });
//     Ok(("", response))
// }
