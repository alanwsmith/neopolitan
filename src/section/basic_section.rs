#![allow(unused)]
use super::Section;
use crate::neo_config::NeoConfig;
use crate::section_category::SectionCategory;
use crate::section_parent::SectionParent;
use nom::{IResult, branch::alt, bytes::complete::tag, combinator::rest};
use serde::{Deserialize, Serialize};

pub fn basic_section<'a>(
    source: &'a str,
    config: &'a NeoConfig,
    parent: &'a SectionParent,
    debug: bool,
) -> IResult<&'a str, Section> {
    let (source, _) = tag("--")(source)?;
    Ok((
        "",
        Section {
            category: SectionCategory::Raw,
            kind: "title".to_string(),
        },
    ))
}
