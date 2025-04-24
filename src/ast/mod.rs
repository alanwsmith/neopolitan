#![allow(unused)]
use crate::neo_config::NeoConfig;
use crate::neo_parser::NeoParser;
use crate::section::Section;
use anyhow::Error;
use nom::Err;
// use nom::error::Error;
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

impl Ast<'_> {
    pub fn new_from_source<'a>(
        source: &'a str,
        config: &'a NeoConfig,
        debug: bool,
    ) -> Ast<'a> {
        match NeoParser::parse(source, config, debug) {
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
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_test() {
        let config = NeoConfig::default();
        let source = include_str!("test-data/basic-example.neo");
        if let Ast::Ok(sections) = Ast::new_from_source(source, &config, false)
        {
            dbg!(&sections);
            assert_eq!(1, sections.len());
        }
    }
}
