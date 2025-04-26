use crate::config::Config;
use crate::section::Section;
use crate::section::section;
use crate::section_parent::SectionParent;
use nom::IResult;
use nom::Parser;
use nom::character::complete::multispace0;
use nom::multi::many1;
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
            many1(|src| section(src, config, parent, debug)).parse(source)?;
        Ok((source, sections))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use minijinja::syntax::SyntaxConfig;
    use minijinja::{Environment, Value, context};
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;
    // use serde_json;

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
            // let mut env = Environment::new();
            // env.set_syntax(
            //     SyntaxConfig::builder()
            //         .block_delimiters("[!", "!]")
            //         .variable_delimiters("[@", "@]")
            //         .comment_delimiters("[#", "#]")
            //         .build()
            //         .unwrap(),
            // );
            // env.add_template("t", include_str!("../dev/template.neoj"))
            //     .unwrap();
            // match env.get_template("t") {
            //     Ok(template) => {
            //         match template.render(
            //             context!(page => Value::from_serialize(&sections)),
            //         ) {
            //             Ok(output) => {
            //                 let tmp_html_path =
            //                     PathBuf::from("dev-output/basic/index.html");
            //                 let _ = std::fs::write(tmp_html_path, output);
            //             }
            //             Err(e) => {
            //                 dbg!(e);
            //                 ()
            //             }
            //         }
            //     }
            //     Err(e) => {
            //         dbg!(e);
            //         ()
            //     }
            // }

            // let tmp_json_path = PathBuf::from("dev-output/basic/ast.json");
            // let json = serde_json::to_string_pretty(&sections).unwrap();
            // std::fs::write(tmp_json_path, json);
            assert_eq!(1, sections.len());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn output_test() {
        let config = Config::default();
        let source = include_str!("test-data/output-example.neo");
        if let Ast::Ok(sections) = Ast::new_from_source(source, &config, false)
        {
            let mut env = Environment::new();
            env.set_syntax(
                SyntaxConfig::builder()
                    .block_delimiters("[!", "!]")
                    .variable_delimiters("[@", "@]")
                    .comment_delimiters("[#", "#]")
                    .build()
                    .unwrap(),
            );
            env.add_template(
                "page.neoj",
                include_str!("../dev-templates/page.neoj"),
            )
            .unwrap();
            env.add_template(
                "basic.neoj",
                include_str!("../dev-templates/basic.neoj"),
            )
            .unwrap();
            env.add_template(
                "block.neoj",
                include_str!("../dev-templates/block.neoj"),
            )
            .unwrap();
            match env.get_template("page.neoj") {
                Ok(template) => {
                    match template.render(
                        context!(page => Value::from_serialize(&sections)),
                    ) {
                        Ok(output) => {
                            let tmp_html_path =
                                PathBuf::from("dev-output/basic/index.html");
                            let _ = std::fs::write(tmp_html_path, output);
                        }
                        Err(e) => {
                            dbg!(e);
                            ()
                        }
                    }
                }
                Err(e) => {
                    dbg!(e);
                    ()
                }
            }
            // let tmp_json_path = PathBuf::from("dev-output/basic/ast.json");
            // let json = serde_json::to_string_pretty(&sections).unwrap();
            // std::fs::write(tmp_json_path, json);
            // assert_eq!(1, sections.len());
        } else {
            assert!(false);
        }
    }
}
