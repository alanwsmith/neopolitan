use crate::get_paragraphs::get_paragraphs;
use crate::section::Section;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until1;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::IResult;
use std::collections::HashMap;

pub fn ordered_list_item_splitter(source: &str) -> IResult<&str, Section> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("- ")(source)?;
    let (source, text) = alt((take_until1("\n\n"), rest))(source)?;
    let (_, paragraphs) = get_paragraphs(text)?;
    let list_item = Section::ORDERED_LIST_ITEM {
        attributes: HashMap::new(),
        children: paragraphs,
    };
    Ok((source, list_item))
}

pub fn get_ordered_list(source: &str) -> IResult<&str, Section> {
    let (_, list_items) = many_till(ordered_list_item_splitter, eof)(source)?;
    let list = Section::ORDERED_LIST {
        attributes: HashMap::new(),
        children: list_items.0,
    };
    Ok((source, list))
}
