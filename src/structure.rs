#![allow(warnings)]
// use crate::spec::Spec;
use crate::chunk::*;
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
            children: Some(vec![Chunk::H1 {
                attributes: None,
                children: Some(vec![Chunk::Text {
                    attributes: None,
                    value: Some("Kickoff".to_string()),
                }]),
            }]),
        }]),
    });
    Ok(("", response))
}
