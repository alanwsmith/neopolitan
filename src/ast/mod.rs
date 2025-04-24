use crate::neo_config::NeoConfig;
use crate::neo_parser::NeoParser;
use crate::section::Section;
use anyhow;
use nom::Err;

pub enum Ast<'a> {
    Ok(Vec<Section>),
    Error(Err<nom::error::Error<&'a str>>),
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
                    // TODO: Make this an error if
                    // the full thing wasn't parsed.
                    Ast::Ok(results.1)
                } else {
                    Ast::Ok(results.1)
                }
            }
            Err(e) => Ast::Error(e),
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
            assert_eq!(1, sections.len());
        }
    }
}
