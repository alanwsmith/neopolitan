#![allow(unused)]
use crate::neo_config::NeoConfig;
use crate::section_category::SectionCategory;
use nom::IResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Section {
    pub category: SectionCategory,
    pub kind: String,
}

pub fn parse_section<'a>(
    source: &'a str,
    config: &'a NeoConfig,
    debug: bool,
) -> IResult<&'a str, Section> {
    Ok((
        "",
        Section {
            category: SectionCategory::Raw,
            kind: "title".to_string(),
        },
    ))
}
