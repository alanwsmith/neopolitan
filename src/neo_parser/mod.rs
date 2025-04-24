#![allow(unused)]
use crate::section::parse_section;
use crate::section_category::SectionCategory;
use crate::section_parent::SectionParent;
use crate::{neo_config::NeoConfig, section::Section};
use anyhow::{Error, Result};
use nom::multi::many1;
use nom::{Finish, IResult};
// use nom::{Err, Parser};
use nom::Parser;
use nom::character::complete::multispace0;
// use nom::multi::many1;
// use nom_supreme::ParserExt;
// use nom_supreme::error::ErrorTree;
// use nom_supreme::final_parser::final_parser;
// use anyhow::Error;

pub struct NeoParser {}

impl<'a> NeoParser {
    pub fn parse(
        source: &'a str,
        config: &'a NeoConfig,
        parent: &'a SectionParent,
        debug: bool,
    ) -> IResult<&'a str, Vec<Section>> {
        let (source, _) = multispace0(source)?;
        let (source, sections) =
            many1(|src| parse_section(src, config, parent, debug))
                .parse(source)?;
        Ok(("", sections))
    }
}
