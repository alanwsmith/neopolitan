#![allow(warnings)]
use crate::chunk::Chunk;
use crate::page::Page;
use crate::process_text::*;
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
    let (source, _) = multispace0(source)?;
    let (source, mut block) = alt((
        tag("-> TITLE").map(|_| Section::TITLE { children: vec![] }),
        tag("-> P").map(|_| Section::P { children: vec![] }),
    ))(source)?;
    match block {
        Section::TITLE { ref mut children } => {
            // Remove leading spaces
            let (source, _) = space0(source)?;
            // Chomp remaining spaces
            let (source, value) = line_ending(source)?;
            // Get the attributes
            let (source, attributes) = many0(attribute_splitter)(source)?;
            // Prep a map and throw the attributes in it
            let mut attribute_map: HashMap<String, String> =
                HashMap::from([("class".to_string(), "title".to_string())]);
            for attribute in attributes {
                let (remainder, key) = take_until(":")(attribute)?;
                let (value, _) = tag(":")(remainder)?;
                attribute_map.insert(key.trim().to_string(), value.trim().to_string());
            }
            // Get the rest of the content
            let (return_content, content) = alt((take_until("\n-> "), rest))(source)?;
            // Get the title
            let (remainder, title) = alt((take_until("\n\n"), rest))(content)?;
            // Chomp spaces
            let (remainder, _) = multispace0(remainder)?;
            // Get paragraphs
            let (remainder, mut paragraphs) =
                separated_list0(tag("\n\n"), take_until("\n\n"))(remainder)?;
            paragraphs.push(remainder.trim());
            // Push the title on
            // TODO: Move this up to right after
            // the title is captured.
            children.push(Chunk::H1 {
                attributes: attribute_map,
                children: vec![Chunk::Text {
                    value: title.trim().to_string(),
                }],
            });
            // Add any paragraphs
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
            // Chomp leading spaces
            let (source, _) = space0(source)?;
            // Clear to the end of the line
            let (source, value) = line_ending(source)?;
            // Get the attributes
            let (source, attributes) = many0(attribute_splitter)(source)?;
            // Prep a map and throw the attributes in it
            let mut attribute_map: HashMap<String, String> = HashMap::from([]);
            for attribute in attributes {
                let (remainder, key) = take_until(":")(attribute)?;
                let (value, _) = tag(":")(remainder)?;
                attribute_map.insert(key.trim().to_string(), value.trim().to_string());
            }
            // Get the rest of the content
            let (return_content, content) = alt((take_until("\n-> "), rest))(source)?;
            // Clear leading spaces
            let (content, _) = multispace0(content)?;
            // Get the paragraphs
            let (remainder, mut paragraphs) =
                separated_list0(tag("\n\n"), take_until("\n\n"))(content)?;
            paragraphs.push(remainder);
            // Push paragraphs onto the vec
            for paragraph in paragraphs.iter() {
                let mut local_attributes: HashMap<String, String> = HashMap::new();
                for (attribute_key, attribute_value) in &attribute_map {
                    local_attributes.insert(attribute_key.to_string(), attribute_value.to_string());
                }
                children.push(Chunk::P {
                    attributes: local_attributes,
                    children: vec![Chunk::Text {
                        value: paragraph.trim().to_string(),
                    }],
                });
            }
            Ok((return_content, block))
        }
        _ => {
            let section = Section::PLACEHOLDER;
            Ok(("", section))
        }
    }
}

pub fn section_dev(source: &str) -> IResult<&str, Section> {
    let (source, _) = multispace0(source)?;
    let (source, mut block) = alt((
        tag("-> TITLE").map(|_| Section::TITLE { children: vec![] }),
        tag("-> P").map(|_| Section::P { children: vec![] }),
    ))(source)?;
    match block {
        Section::TITLE { ref mut children } => {
            // Remove leading spaces
            let (source, _) = space0(source)?;
            // Chomp remaining spaces
            let (source, value) = line_ending(source)?;
            // Get the attributes
            let (source, attributes) = many0(attribute_splitter)(source)?;
            // Prep a map and throw the attributes in it
            let mut attribute_map: HashMap<String, String> =
                HashMap::from([("class".to_string(), "title".to_string())]);
            for attribute in attributes {
                let (remainder, key) = take_until(":")(attribute)?;
                let (value, _) = tag(":")(remainder)?;
                attribute_map.insert(key.trim().to_string(), value.trim().to_string());
            }
            // Get the rest of the content
            let (return_content, content) = alt((take_until("\n-> "), rest))(source)?;
            // Get the title
            let (remainder, title) = alt((take_until("\n\n"), rest))(content)?;
            // Chomp spaces
            let (remainder, _) = multispace0(remainder)?;
            // Get paragraphs
            let (remainder, mut paragraphs) =
                separated_list0(tag("\n\n"), take_until("\n\n"))(remainder)?;
            paragraphs.push(remainder.trim());
            // Push the title on
            // TODO: Move this up to right after
            // the title is captured.
            children.push(Chunk::H1 {
                attributes: attribute_map,
                children: vec![Chunk::Text {
                    value: title.trim().to_string(),
                }],
            });
            // Add any paragraphs
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
            // Chomp leading spaces
            let (source, _) = space0(source)?;
            // Clear to the end of the line
            let (source, value) = line_ending(source)?;
            // Get the attributes
            let (source, attributes) = many0(attribute_splitter)(source)?;
            // Prep a map and throw the attributes in it
            let mut attribute_map: HashMap<String, String> = HashMap::from([]);
            for attribute in attributes {
                let (remainder, key) = take_until(":")(attribute)?;
                let (value, _) = tag(":")(remainder)?;
                attribute_map.insert(key.trim().to_string(), value.trim().to_string());
            }
            // Get the rest of the content
            let (return_content, content) = alt((take_until("\n-> "), rest))(source)?;
            // Clear leading spaces
            let (content, _) = multispace0(content)?;
            // Get the paragraphs
            let (remainder, mut paragraphs) =
                separated_list0(tag("\n\n"), take_until("\n\n"))(content)?;
            paragraphs.push(remainder);
            // Push paragraphs onto the vec
            for paragraph in paragraphs.iter() {
                let mut local_attributes: HashMap<String, String> = HashMap::new();
                for (attribute_key, attribute_value) in &attribute_map {
                    local_attributes.insert(attribute_key.to_string(), attribute_value.to_string());
                }
                let (chunk_remainder, mut chunks) = process_text(paragraph.trim())?;
                if chunk_remainder.is_empty() {
                } else {
                    chunks.push(Chunk::Text {
                        value: chunk_remainder.to_string(),
                    });
                }
                // dbg!(&chunks);
                // Process the text in the thing
                // let (last_child, paragraph_children) = many0(process_text)(paragraph)?;
                children.push(Chunk::P {
                    attributes: local_attributes,
                    // children: process_text(paragraph.trim()).unwrap().1, // children: vec![Chunk::Text {
                    children: chunks,
                });
                // Attempt
                // children.push(process_text(paragraph.trim().as_str()).unwrap().1);
                // dbg!(process_text(paragraph.trim()));
            }
            ////Shameless Green
            //let block = Section::P {
            //    children: vec![Chunk::P {
            //        attributes: HashMap::new(),
            //        children: vec![
            //            Chunk::Text {
            //                value: "The ".to_string(),
            //            },
            //            Chunk::InlineCode {
            //                attributes: None,
            //                language: Some("rust".to_string()),
            //                value: Some("sand".to_string()),
            //            },
            //            Chunk::Text {
            //                value: " drifts".to_string(),
            //            },
            //        ],
            //    }],
            //};

            Ok((return_content, block))
        }
        _ => {
            let section = Section::PLACEHOLDER;
            Ok(("", section))
        }
    }
}
