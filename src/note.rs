#![allow(warnings)]
use crate::attributes::attributes;
use crate::chunk::Chunk;
use crate::code::*;
use crate::page::Page;
use crate::section::*;
use crate::text::*;
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

pub fn note(source: &str) -> IResult<&str, Section> {
    let (source, attributes) = attributes(source)?;
    let (_, paragraphs) = split_parts(source, "\n\n")?;

    let children: Vec<Chunk> = paragraphs
        .iter()
        .map(|p| Chunk::P {
            attributes: None,
            children: text(p).unwrap().1,
        })
        .collect();

    dbg!(paragraphs);

    let expected = Section::NoteSection {
        attributes,
        children: Some(children), // children: Some(vec![
                                  //     Chunk::P {
                                  //         attributes: None,
                                  //         children: Some(vec![Chunk::Text {
                                  //             attributes: None,
                                  //             value: Some("There was a sound".to_string()),
                                  //         }]),
                                  //     },
                                  //     Chunk::P {
                                  //         attributes: None,
                                  //         children: Some(vec![Chunk::Text {
                                  //             attributes: None,
                                  //             value: Some("The leaves were dry".to_string()),
                                  //         }]),
                                  //     },
                                  // ]),
    };

    Ok(("", expected))

    // let (remainder, mut note_parts) = many0(note_part)(source)?;
    // &note_parts.push(remainder);
    // match attributes {
    //     Some(x) => Ok((
    //         "",
    //         Section::NoteSection {
    //             // attributes: Some(x),
    //             attributes: None,
    //             children: Some(
    //                 note_parts
    //                     .iter()
    //                     .map(|p| Chunk::P {
    //                         attributes: None,
    //                         children: text(p).unwrap().1,
    //                     })
    //                     .collect(),
    //             ),
    //         },
    //     )),
    //     None => Ok((
    //         "",
    //         Section::NoteSection {
    //             attributes: None,
    //             children: Some(
    //                 note_parts
    //                     .iter()
    //                     .map(|p| Chunk::P {
    //                         attributes: None,
    //                         children: text(p).unwrap().1,
    //                     })
    //                     .collect(),
    //             ),
    //         },
    //     )),
    // }
}

// fn note_part(source: &str) -> IResult<&str, &str> {
//     let (source, content) = take_until("\n\n")(source)?;
//     let (source, _) = tag("\n\n")(source)?;
//     Ok((source, content))
// }

fn split_parts<'a>(source: &'a str, separator: &'a str) -> IResult<&'a str, Vec<&'a str>> {
    let (remainder, content) = opt(tag(separator))(source)?;
    match content {
        None => {
            let (_, items) = separated_list0(tag(separator), is_not(separator))(remainder)?;
            Ok(("", items))
        }
        Some(..) => {
            let (_, items) = separated_list0(tag(separator), is_not(separator))(remainder)?;
            Ok(("", items))
        }
    }
}
