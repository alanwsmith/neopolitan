#![allow(warnings)]
use crate::chunk::Chunk;
use crate::code::*;
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
    TitleSection {
        children: Vec<Chunk>,
    },
    ParagraphSection {
        children: Vec<Chunk>,
    },
    CodeSection {
        language: Option<String>,
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        children: Vec<Chunk>,
    },
    NoteSection {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        children: Vec<Chunk>,
    },
    Placeholder,
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
        tag("-> NOTE").map(|_| Section::NoteSection {
            attributes: None,
            children: vec![],
        }),
        tag("-> P").map(|_| Section::ParagraphSection { children: vec![] }),
        tag("-> TITLE").map(|_| Section::TitleSection { children: vec![] }),
        tag("-> CODE").map(|_| Section::CodeSection {
            attributes: None,
            language: None,
            children: vec![],
        }),
    ))(source)?;
    match block {
        Section::TitleSection { ref mut children } => {
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
            if attribute_map.is_empty() {
                children.push(Chunk::H1 {
                    attributes: None,
                    children: Some(vec![Chunk::Text {
                        value: title.trim().to_string(),
                    }]),
                });
            } else {
                children.push(Chunk::H1 {
                    attributes: Some(attribute_map),
                    children: Some(vec![Chunk::Text {
                        value: title.trim().to_string(),
                    }]),
                });
            }
            for paragraph in paragraphs.iter() {
                if paragraph.is_empty() {
                } else {
                    children.push(Chunk::P {
                        attributes: None,
                        children: Some(vec![Chunk::Text {
                            value: paragraph.trim().to_string(),
                        }]),
                    });
                }
            }
            Ok((return_content, block))
        }
        Section::ParagraphSection { ref mut children } => {
            let (source, _) = space0(source)?;
            let (source, value) = line_ending(source)?;
            let (source, attributes) = many0(attribute_splitter)(source)?;
            let mut attribute_map: HashMap<String, String> = HashMap::from([]);
            for attribute in attributes {
                let (remainder, key) = take_until(":")(attribute)?;
                let (value, _) = tag(":")(remainder)?;
                attribute_map.insert(key.trim().to_string(), value.trim().to_string());
            }
            let (return_content, content) = alt((take_until("\n-> "), rest))(source)?;
            let (content, _) = multispace0(content)?;
            let (remainder, mut paragraphs) =
                separated_list0(tag("\n\n"), take_until("\n\n"))(content)?;
            paragraphs.push(remainder);
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
                if local_attributes.is_empty() {
                    children.push(Chunk::P {
                        attributes: None,
                        children: Some(chunks),
                    });
                } else {
                    children.push(Chunk::P {
                        attributes: Some(local_attributes),
                        children: Some(chunks),
                    });
                }
            }
            Ok((return_content, block))
        }
        Section::CodeSection {
            ref mut children,
            ref mut attributes,
            ref mut language,
        } => {
            let (return_content, source) = alt((take_until("\n-> "), rest))(source)?;
            let (_, block) = code(source)?;
            Ok((return_content, block))
        }

        _ => {
            let block = Section::Placeholder;
            Ok(("", block))
        }
    }
}
