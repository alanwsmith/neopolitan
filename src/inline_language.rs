#![allow(warnings)]
use crate::chunk::Chunk;
use crate::language::*;
use crate::page::Page;
use crate::parse_text_attributes::parse_text_attributes;
use crate::split::split;
use crate::tag_attributes::*;
use crate::text_attributes::*;
use nom::branch::alt;
use nom::bytes::complete::is_not;
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
use nom::combinator::opt;
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

pub fn inline_language(source: &str) -> IResult<&str, Option<String>> {
    // dbg!(&source);
    let (_, parts) = split(source, "|")?;
    if parts.len() == 0 {
        Ok(("", None))
    } else {
        Ok(("", Some(parts[0].trim().to_string())))
    }

    // // let (remainder, content) = alt((take_until("\n\n"), rest))(source)?;
    // let (content, _) = multispace0(source)?;
    // dbg!(&content);
    // // dbg!(&remainder);
    // let (_, content) = alt((
    //     tuple((tag(">>"), not_line_ending, rest)),
    //     tuple((rest, rest, rest)),
    // ))(content)?;
    // if content.1.is_empty() {
    //     Ok(("", None))
    // } else {
    //     let (a, b) = multispace0(content.1)?;
    //     let (c, d) = alt((tuple((take_until(": "), tag(": "))), tuple((rest, rest))))(a)?;
    //     if d.1.is_empty() {
    //         Ok(("", Some(d.0.trim().to_string())))
    //     } else {
    //         Ok(("", None))
    //     }
}

// fn split<'a>(source: &'a str, separator: &'a str) -> IResult<&'a str, Vec<&'a str>> {
//     let (remainder, content) = opt(tag(separator))(source)?;
//     match content {
//         None => {
//             let (_, items) = separated_list0(tag(separator), is_not(separator))(remainder)?;
//             Ok(("", items))
//         }
//         Some(..) => {
//             let (_, items) = separated_list0(tag(separator), is_not(separator))(remainder)?;
//             Ok(("", items))
//         }
//     }
// }
