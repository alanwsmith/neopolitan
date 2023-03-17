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

#[derive(Debug, PartialEq)]
pub enum Section {
    TITLE { children: Vec<Chunk> },
    P { children: Vec<Chunk> },
    PLACEHOLDER,
}

fn attribute_splitter(source: &str) -> IResult<&str, &str> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag(">> ")(source)?;
    let (source, value) = not_line_ending(source)?;
    let (source, _) = multispace0(source)?;
    Ok((source, value))
}

pub fn section(source: &str) -> IResult<&str, Section> {
    // dbg!("-- SECTION START");
    let (source, _) = multispace0(source)?;
    let (source, mut block) = alt((
        tag("-> TITLE").map(|_| Section::TITLE { children: vec![] }),
        tag("-> P").map(|_| Section::P { children: vec![] }),
    ))(source)?;
    match block {
        Section::TITLE { ref mut children } => {
            // dbg!("-- SECTION:TITLE");
            let (source, _) = space0(source)?;
            let (source, value) = line_ending(source)?;
            let (source, attributes) = many0(attribute_splitter)(source)?;
            let mut attribute_map: HashMap<String, String> =
                HashMap::from([("class".to_string(), "title".to_string())]);
            for attribute in attributes {
                let (remainder, key) = take_until(":")(attribute)?;
                let (value, _) = tag(":")(remainder)?;
                attribute_map.insert(key.trim().to_string(), value.trim().to_string());
            }
            let (return_content, content) = alt((take_until("\n-> "), rest))(source)?;
            let (remainder, title) = alt((take_until("\n\n"), rest))(content)?;
            let (remainder, _) = multispace0(remainder)?;
            let (remainder, mut paragraphs) =
                separated_list0(tag("\n\n"), take_until("\n\n"))(remainder)?;
            paragraphs.push(remainder.trim());
            children.push(Chunk::H1 {
                attributes: attribute_map,
                children: vec![Chunk::Text {
                    value: title.trim().to_string(),
                }],
            });
            for paragraph in paragraphs.iter() {
                if paragraph.is_empty() {
                } else {
                    children.push(Chunk::P {
                        attributes: HashMap::new(),
                        children: vec![Chunk::Text {
                            value: paragraph.trim().to_string(),
                        }],
                    });
                }
            }
            Ok((return_content, block))
        }
        Section::P { ref mut children } => {
            let attribute_map = HashMap::new();
            // dbg!("-- SECTION:P");
            // dbg!(&source);
            let (source, _) = space0(source)?;
            let (source, value) = line_ending(source)?;
            let (source, _) = space0(source)?;
            let (text, value) = line_ending(source)?;

            // dbg!("--------------------------");
            // dbg!(&source);
            // dbg!(&text);
            // dbg!(&value);
            // dbg!("--------------------------");

            let (return_content, content) = alt((take_until("\n-> "), rest))(source)?;

            // dbg!(&content);
            // dbg!("--------------------------");

            children.push(Chunk::P {
                attributes: attribute_map,
                children: vec![Chunk::Text {
                    value: text.trim().to_string(),
                }],
            });
            Ok((return_content, block))
        }
        _ => {
            let section = Section::PLACEHOLDER;
            Ok(("", section))
        }
    }
}

pub fn section_dev(source: &str) -> IResult<&str, Section> {
    // dbg!("-- SECTION START");
    let (source, _) = multispace0(source)?;
    let (source, mut block) = alt((
        tag("-> TITLE").map(|_| Section::TITLE { children: vec![] }),
        tag("-> P").map(|_| Section::P { children: vec![] }),
    ))(source)?;
    match block {
        Section::TITLE { ref mut children } => {
            // dbg!("-- SECTION:TITLE");
            let (source, _) = space0(source)?;
            let (source, value) = line_ending(source)?;
            let (source, attributes) = many0(attribute_splitter)(source)?;
            let mut attribute_map: HashMap<String, String> =
                HashMap::from([("class".to_string(), "title".to_string())]);
            for attribute in attributes {
                let (remainder, key) = take_until(":")(attribute)?;
                let (value, _) = tag(":")(remainder)?;
                attribute_map.insert(key.trim().to_string(), value.trim().to_string());
            }
            let (return_content, content) = alt((take_until("\n-> "), rest))(source)?;
            let (remainder, title) = alt((take_until("\n\n"), rest))(content)?;
            let (remainder, _) = multispace0(remainder)?;
            let (remainder, mut paragraphs) =
                separated_list0(tag("\n\n"), take_until("\n\n"))(remainder)?;
            paragraphs.push(remainder.trim());
            children.push(Chunk::H1 {
                attributes: attribute_map,
                children: vec![Chunk::Text {
                    value: title.trim().to_string(),
                }],
            });
            for paragraph in paragraphs.iter() {
                if paragraph.is_empty() {
                } else {
                    children.push(Chunk::P {
                        attributes: HashMap::new(),
                        children: vec![Chunk::Text {
                            value: paragraph.trim().to_string(),
                        }],
                    });
                }
            }
            Ok((return_content, block))
        }
        Section::P { ref mut children } => {
            // let attribute_map: HashMap<String, String> = HashMap::new();
            // dbg!("-- SECTION:P");
            // dbg!(&source);
            let (source, _) = space0(source)?;
            let (source, value) = line_ending(source)?;
            let (source, _) = space0(source)?;
            let (text, value) = line_ending(source)?;
            dbg!("--------------------------");
            dbg!(&source);
            dbg!(&text);
            dbg!(&value);
            dbg!("--------------------------");
            let (return_content, content) = alt((take_until("\n-> "), rest))(source)?;
            let (content, _) = multispace0(content)?;
            let (remainder, mut paragraphs) =
                separated_list0(tag("\n\n"), take_until("\n\n"))(content)?;
            paragraphs.push(remainder);
            for paragraph in paragraphs.iter() {
                children.push(Chunk::P {
                    attributes: HashMap::new(),
                    children: vec![Chunk::Text {
                        value: paragraph.trim().to_string(),
                    }],
                });
            }
            dbg!("--------------------------");
            dbg!(&content);
            dbg!(&paragraphs);
            dbg!("--------------------------");
            dbg!("--------------------------");
            dbg!(&return_content);
            dbg!("--------------------------");
            Ok((return_content, block))
        }
        _ => {
            let section = Section::PLACEHOLDER;
            Ok(("", section))
        }
    }
}
