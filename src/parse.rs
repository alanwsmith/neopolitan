#![allow(warnings)]
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until1;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many1;
use nom::multi::many_till;
use nom::IResult;
use nom::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Page {
    attributes: HashMap<String, String>,
    children: Vec<Section>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Section {
    ATTRIBUTES {
        raw: HashMap<String, String>,
    },
    Title {
        attributes: HashMap<String, String>,
        children: Vec<Content>,
    },
    H1 {
        attributes: HashMap<String, String>,
        children: Vec<Content>,
    },
    H2 {
        attributes: HashMap<String, String>,
        children: Vec<Content>,
    },
    H3 {
        attributes: HashMap<String, String>,
        children: Vec<Content>,
    },
    H4 {
        attributes: HashMap<String, String>,
        children: Vec<Content>,
    },
    H5 {
        attributes: HashMap<String, String>,
        children: Vec<Content>,
    },
    H6 {
        attributes: HashMap<String, String>,
        children: Vec<Content>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Marker {
    ATTRIBUTES,
    Title,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Content {
    Text { value: String },
    PlainText { value: String },
}

pub fn get_title(source: &str) -> IResult<&str, Section> {
    Ok((
        "",
        Section::Title {
            attributes: HashMap::new(),
            children: vec![get_text(source).unwrap().1],
        },
    ))
}

pub fn get_text(source: &str) -> IResult<&str, Content> {
    let (source, _) = multispace0(source)?;
    let (source, content) = not_line_ending(source)?;
    Ok((
        source,
        Content::PlainText {
            value: content.trim().to_string(),
        },
    ))
}

pub fn h1(source: &str) -> IResult<&str, Section> {
    Ok((
        "",
        Section::H1 {
            attributes: HashMap::new(),
            children: vec![get_text(source).unwrap().1],
        },
    ))
}

pub fn h2(source: &str) -> IResult<&str, Section> {
    Ok((
        "",
        Section::H2 {
            attributes: HashMap::new(),
            children: vec![get_text(source).unwrap().1],
        },
    ))
}
pub fn h3(source: &str) -> IResult<&str, Section> {
    Ok((
        "",
        Section::H3 {
            attributes: HashMap::new(),
            children: vec![get_text(source).unwrap().1],
        },
    ))
}
pub fn h4(source: &str) -> IResult<&str, Section> {
    Ok((
        "",
        Section::H4 {
            attributes: HashMap::new(),
            children: vec![get_text(source).unwrap().1],
        },
    ))
}
pub fn h5(source: &str) -> IResult<&str, Section> {
    Ok((
        "",
        Section::H5 {
            attributes: HashMap::new(),
            children: vec![get_text(source).unwrap().1],
        },
    ))
}

pub fn h6(source: &str) -> IResult<&str, Section> {
    Ok((
        "",
        Section::H6 {
            attributes: HashMap::new(),
            children: vec![get_text(source).unwrap().1],
        },
    ))
}

pub fn section(source: &str) -> IResult<&str, Section> {
    let (source, _) = multispace0(source)?;
    let (source, section_type) = alt((
        tag("-> TITLE").map(|_| Marker::Title),
        tag("-> H1").map(|_| Marker::H1),
        tag("-> H2").map(|_| Marker::H2),
        tag("-> H3").map(|_| Marker::H3),
        tag("-> H4").map(|_| Marker::H4),
        tag("-> H5").map(|_| Marker::H5),
        tag("-> H6").map(|_| Marker::H6),
        tag("-> ATTRIBUTES").map(|_| Marker::ATTRIBUTES),
    ))(source)?;
    let (source, content) = alt((take_until1("\n\n-> "), rest))(source)?;
    match section_type {
        // Marker::ATTRIBUTES => Ok((source, Section::ATTRIBUTES)),
        Marker::ATTRIBUTES => Ok((source, prep_attributes(content).unwrap().1)),
        Marker::H1 => Ok((source, h1(content).unwrap().1)),
        Marker::H2 => Ok((source, h2(content).unwrap().1)),
        Marker::H3 => Ok((source, h3(content).unwrap().1)),
        Marker::H4 => Ok((source, h4(content).unwrap().1)),
        Marker::H5 => Ok((source, h5(content).unwrap().1)),
        Marker::H6 => Ok((source, h6(content).unwrap().1)),
        Marker::Title => Ok((source, get_title(content).unwrap().1)),
    }
}

pub fn get_attribute(source: &str) -> IResult<&str, (String, String)> {
    dbg!(&source);
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("-> ")(source)?;
    let (source, key) = take_until1(": ")(source)?;
    let (source, _) = tag(":")(source)?;
    let (source, _) = multispace0(source)?;
    let (source, value) = take_until1("\n")(source)?;
    let (source, _) = multispace0(source)?;
    Ok((source, (key.trim().to_string(), value.trim().to_string())))
}

pub fn prep_attributes(source: &str) -> IResult<&str, Section> {
    let (source, pairs) = many_till(get_attribute, eof)(source)?;
    let mut attrs: HashMap<String, String> = HashMap::new();
    for pair in pairs.0.iter() {
        attrs.insert(pair.0.to_string(), pair.1.to_string());
    }
    let the_attrs = Section::ATTRIBUTES { raw: attrs };
    dbg!(&the_attrs);
    Ok((source, the_attrs))
}

pub fn get_sections(source: &str) -> IResult<&str, Vec<Section>> {
    let (source, sections) = many_till(section, eof)(source)?;
    Ok((source, sections.0))
}

pub fn parse(source: &str) -> Page {
    let page = Page {
        attributes: HashMap::new(),
        children: get_sections(source).unwrap().1,
    };
    page
}

pub fn parse_dev(source: &str) -> Page {
    let raw_sections = get_sections(source).unwrap().1;
    let mut sections: Vec<Section> = vec![];
    let mut attrs: HashMap<String, String> = HashMap::new();
    for raw_section in raw_sections {
        match raw_section {
            Section::ATTRIBUTES { raw } => attrs = raw,
            _ => {
                sections.push(raw_section);
            }
        }
    }

    // let mut attrs = HashMap::new();
    // attrs.insert("date".to_string(), "2023-03-12 17:07:23".to_string());
    // attrs.insert("id".to_string(), "1234asdf".to_string());
    // attrs.insert("type".to_string(), "test".to_string());

    let page = Page {
        attributes: attrs,
        children: sections,
    };
    page
}
