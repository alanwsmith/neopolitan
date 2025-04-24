#![allow(unused)]
use crate::section_category::SectionCategory;
use crate::{neo_config::NeoConfig, section::Section};
use nom::{Finish, IResult};
// use nom::{Err, Parser};
use nom::Parser;
use nom::character::complete::multispace0;
// use nom::multi::many1;
// use nom_supreme::ParserExt;
// use nom_supreme::error::ErrorTree;
// use nom_supreme::final_parser::final_parser;

pub struct NeoParser {}

impl<'a> NeoParser {
    pub fn parse(
        source: &'a str,
        config: &'a NeoConfig,
        debug: bool,
    ) -> IResult<&'a str, Vec<Section>> {
        decend(source, config, debug)
    }
}

fn decend<'a>(
    source: &'a str,
    config: &'a NeoConfig,
    debug: bool,
) -> IResult<&'a str, Vec<Section>> {
    let (source, _) = multispace0(source)?;
    Ok((
        "",
        vec![Section {
            category: SectionCategory::Raw,
            kind: "title".to_string(),
        }],
    ))
}
