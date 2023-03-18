use crate::attributes::*;
use crate::chunk::Chunk;
use crate::code::*;
use crate::list::*;
use crate::note::*;
use crate::process_text::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::combinator::rest;
use nom::multi::many0;
use nom::multi::separated_list0;
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Section {
    // Probably change the non-option stuff
    // to options at some point
    CodeSection {
        language: Option<String>,
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        children: Vec<Chunk>,
    },
    ListSection {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        children: Option<Vec<Chunk>>,
    },
    NoteSection {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        children: Option<Vec<Chunk>>,
    },
    ParagraphSection {
        children: Vec<Chunk>,
    },
    TitleSection {
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
        tag_no_case("-> NOTE").map(|_| Section::NoteSection {
            attributes: None,
            children: None,
        }),
        tag_no_case("-> P").map(|_| Section::ParagraphSection { children: vec![] }),
        tag_no_case("-> TITLE").map(|_| Section::TitleSection { children: vec![] }),
        tag_no_case("-> LIST").map(|_| Section::ListSection {
            attributes: None,
            children: None,
        }),
        tag_no_case("-> CODE").map(|_| Section::CodeSection {
            attributes: None,
            language: None,
            children: vec![],
        }),
    ))(source)?;
    match block {
        Section::TitleSection { ref mut children } => {
            let (source, _) = space0(source)?;
            let (source, _) = line_ending(source)?;
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
            let (source, attribute_list) = attributes(source)?;
            let (return_content, content) = alt((take_until("\n-> "), rest))(source)?;
            let (content, _) = multispace0(content)?;
            let (remainder, mut paragraphs) =
                separated_list0(tag("\n\n"), take_until("\n\n"))(content)?;
            paragraphs.push(remainder);
            for paragraph in paragraphs.iter() {
                children.push(Chunk::P {
                    attributes: attribute_list.clone(),
                    children: text(paragraph).unwrap().1,
                });
            }
            Ok((return_content, block))
        }
        Section::CodeSection {
            children: _children,
            attributes: _attributes,
            language: _language,
        } => {
            let (return_content, source) = alt((take_until("\n-> "), rest))(source)?;
            let (_, block) = code(source)?;
            Ok((return_content, block))
        }
        Section::NoteSection {
            children: _children,
            attributes: _attributes,
        } => {
            let (return_content, source) = alt((take_until("\n-> "), rest))(source)?;
            let (_, block) = note(source)?;
            Ok((return_content, block))
        }
        Section::ListSection {
            children: _children,
            attributes: _attributes,
        } => {
            let (return_content, source) = alt((take_until("\n-> "), rest))(source)?;
            let (_, block) = list(source)?;
            Ok((return_content, block))
        }
        _ => {
            let block = Section::Placeholder;
            Ok(("", block))
        }
    }
}
