#![allow(warnings)]
use crate::chunk::Chunk;
use crate::page::Page;
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
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::Err;
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;

// this is the one for text attributes
// the other one is for attributes for
// sections: TODO: Rename the other
// one to `section_attributes``
//

pub fn text_attributes(
    source: &str,
) -> IResult<&str, Option<Vec<(Option<String>, Option<String>)>>> {
    let response: Option<Vec<(Option<String>, Option<String>)>> = Some(vec![
        (Some("rust".to_string()), None),
        (Some("class".to_string()), Some("sail".to_string())),
    ]);
    Ok(("", response))
}
