#![allow(unused)]
use crate::section::parse_section;
use crate::section_category::SectionCategory;
use crate::section_parent::SectionParent;
use crate::{config::Config, section::Section};
use anyhow::{Error, Result};
use nom::Parser;
use nom::character::complete::multispace0;
use nom::multi::many1;
use nom::{Finish, IResult};

pub struct Parser {}

impl<'a> Parser {
    pub fn parse_ast(
        source: &'a str,
        config: &'a Config,
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
