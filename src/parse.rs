#![allow(warnings)]
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until1;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::rest;
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
    Title {
        attributes: HashMap<String, String>,
        children: Vec<Content>,
    },
    H2 {
        attributes: HashMap<String, String>,
        children: Vec<Content>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Marker {
    Title,
    H2,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Content {
    Text { value: String },
}

pub fn get_title(source: &str) -> Section {
    Section::Title {
        attributes: HashMap::new(),
        children: vec![get_text(source)],
    }
}

pub fn get_text(_source: &str) -> Content {
    Content::Text {
        value: "This is a title".to_string(),
    }
}

pub fn get_sections(source: &str) -> IResult<&str, Vec<Section>> {
    Ok(("", vec![get_title(source)]))
}

pub fn parse(source: &str) -> Page {
    let page = Page {
        attributes: HashMap::new(),
        children: get_sections(source).unwrap().1,
    };
    page
}

pub fn get_title_dev(source: &str) -> IResult<&str, Section> {
    Ok((
        "",
        Section::Title {
            attributes: HashMap::new(),
            children: vec![get_text(source)],
        },
    ))
}

pub fn get_text_dev(_source: &str) -> IResult<&str, Content> {
    Ok((
        "",
        Content::Text {
            value: "This is an H2".to_string(),
        },
    ))
}

pub fn get_h2(source: &str) -> Section {
    Section::H2 {
        attributes: HashMap::new(),
        children: vec![get_text_dev(source).unwrap().1],
    }
}

pub fn get_h2_dev(source: &str) -> IResult<&str, Section> {
    Ok((
        "",
        Section::H2 {
            attributes: HashMap::new(),
            children: vec![get_text_dev(source).unwrap().1],
        },
    ))
}

pub fn section(source: &str) -> IResult<&str, Section> {
    let (source, _) = multispace0(source)?;
    let (source, section_type) = alt((
        tag("-> TITLE").map(|_| Marker::Title),
        tag("-> H2").map(|_| Marker::H2),
    ))(source)?;

    let (source, content) = alt((take_until1("\n\n-> "), rest))(source)?;

    match section_type {
        Marker::Title => Ok((source, get_title_dev(content).unwrap().1)),
        Marker::H2 => Ok((source, get_h2_dev(content).unwrap().1)),
    }

    // Ok((
    //     "",
    //     Section::Title {
    //         attributes: HashMap::new(),
    //         children: vec![get_text("")],
    //     },
    // ))
}

pub fn get_sections_dev(source: &str) -> IResult<&str, Vec<Section>> {
    let (sourc_x, sections) = many_till(section, eof)(source)?;
    Ok(("", vec![get_title(source), get_h2(source)]))
}

pub fn parse_dev(source: &str) -> Page {
    let page = Page {
        attributes: HashMap::new(),
        children: get_sections_dev(source).unwrap().1,
    };
    page
}
