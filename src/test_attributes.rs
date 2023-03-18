#![allow(warnings)]
use crate::attributes::*;
use crate::chunk::Chunk;
use crate::page::Page;
use crate::process_text::*;
use crate::section::*;
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
use nom::combinator::not;
use nom::combinator::rest;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::multi::many0;
use nom::multi::many1;
use nom::multi::many_till;
use nom::multi::separated_list0;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::Err;
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;

#[test]
fn basic_attributes() {
    let source = ">> box: planks\n\nThe salt breeze";
    let expected = Attributes {
        values: Some(vec![(Some("box".to_string()), Some("planks".to_string()))]),
        remainder: Some("The salt breeze".to_string()),
    };
    let result = attributes(source);
    assert_eq!(expected, result.unwrap().1);
}
