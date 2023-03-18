#![allow(warnings)]
use crate::attributes::*;
use crate::chunk::Chunk;
use crate::page::Page;
use crate::section::*;
use crate::text::*;
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
    let expected = Some(vec![(Some("box".to_string()), Some("planks".to_string()))]);
    let result = attributes(source);
    assert_eq!(expected, result.unwrap().1);
}

#[test]
fn full_attributes_langauge_without_other_stuff() {
    let source = ">> python >> creek: wild\n\nRight hand side";
    let expected = Some(vec![
        (Some("python".to_string()), None),
        (Some("creek".to_string()), Some("wild".to_string())),
    ]);
    let result = attributes(source);
    assert_eq!(expected, result.unwrap().1);
}

#[test]
fn no_attributes() {
    let source = "\n\nThe pine tree";
    let expected = None;
    let result = attributes(source);
    assert_eq!(expected, result.unwrap().1);
}

#[test]
fn check_return_values() {
    let source = "\n\nThe pine tree\n\nRunning again";
    let result = attributes(source);
    assert_eq!("The pine tree\n\nRunning again", result.unwrap().0);
}

////////////////////////////////////////////
// This is just a singular attribute instead
// of the collection of attributes like above

#[test]
fn language_without_other_attribute() {
    let source = "rust";
    let expected = (Some("rust".to_string()), None);
    let result = attribute(source);
    assert_eq!(expected, result.unwrap().1);
}
