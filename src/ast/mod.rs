#![allow(unused)]
use crate::config::Config;
use crate::section::Section;
use crate::section::parse_section;
use crate::section_category::SectionCategory;
use crate::section_parent::SectionParent;
use anyhow::{Error, Result};
use nom::Err;
use nom::Parser;
use nom::character::complete::multispace0;
use nom::multi::many1;
use nom::{Finish, IResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Ast<'a> {
    Error {
        message: String,
        remainder: String,
    },
    Incomplete {
        parsed: Vec<Section>,
        remainder: &'a str,
    },
    Ok(Vec<Section>),
}

impl<'a> Ast<'_> {
    pub fn new_from_source(
        source: &'a str,
        config: &'a Config,
        debug: bool,
    ) -> Ast<'a> {
        match Ast::parse_ast(source, config, &SectionParent::Page, debug) {
            Ok(results) => {
                if results.0 == "" {
                    Ast::Ok(results.1)
                } else {
                    Ast::Incomplete {
                        parsed: results.1,
                        remainder: results.0,
                    }
                }
            }
            Err(_e) => Ast::Error {
                message: "TODO: Put message here".to_string(),
                remainder: "TODO: Put remainder here".to_string(),
            },
        }
    }

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

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json;

    #[test]
    fn basic_test() {
        let config = Config::default();
        let source = include_str!("test-data/basic-example.neo");
        if let Ast::Ok(sections) = Ast::new_from_source(source, &config, false)
        {
            // println!("{}", serde_json::to_string_pretty(&sections).unwrap());
            assert_eq!(1, sections.len());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn span_test() {
        let config = Config::default();
        let source = include_str!("test-data/span-test.neo");
        if let Ast::Ok(sections) = Ast::new_from_source(source, &config, false)
        {
            println!("{}", serde_json::to_string_pretty(&sections).unwrap());
            assert_eq!(1, sections.len());
        } else {
            assert!(false);
        }
    }
}
