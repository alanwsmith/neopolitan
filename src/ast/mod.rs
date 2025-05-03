use crate::block::Block;
use crate::block::block;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use nom::IResult;
use nom::Parser;
use nom::character::complete::multispace0;
use nom::multi::many1;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Ast {
    Error {
        message: String,
        remainder: String,
    },
    Incomplete {
        parsed: Vec<Block>,
        remainder: String,
    },
    Ok {
        blocks: Vec<Block>,
    },
}

impl<'a> Ast {
    pub fn new_from_source(source: &'a str, config: &'a Config) -> Ast {
        match Ast::parse_ast(source, config, &BlockParent::Page) {
            Ok(results) => {
                if results.0 == "" {
                    Ast::Ok { blocks: results.1 }
                } else {
                    Ast::Incomplete {
                        parsed: results.1,
                        remainder: results.0.to_string(),
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
        parent: &'a BlockParent,
    ) -> IResult<&'a str, Vec<Block>> {
        let (source, _) = multispace0(source)?;
        let (source, blocks) =
            many1(|src| block(src, config, parent)).parse(source)?;
        Ok((source, blocks))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    // use serde_json;

    #[test]
    fn basic_test() {
        let config = Config::default();
        let source = include_str!("test-data/basic-example.neo");
        if let Ast::Ok { blocks } = Ast::new_from_source(source, &config) {
            assert_eq!(1, blocks.len());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn span_test() {
        let config = Config::default();
        let source = include_str!("test-data/span-test.neo");
        match Ast::new_from_source(source, &config) {
            Ast::Ok { blocks } => assert_eq!(1, blocks.len()),
            Ast::Error { message, remainder } => {
                dbg!(message);
                dbg!(remainder);
                assert!(false)
            }
            Ast::Incomplete { parsed, remainder } => {
                dbg!(parsed);
                dbg!(remainder);
                assert!(false)
            }
        }
        // }
        // {
        //     // let mut env = Environment::new();
        //     // env.set_syntax(
        //     //     SyntaxConfig::builder()
        //     //         .block_delimiters("[!", "!]")
        //     //         .variable_delimiters("[@", "@]")
        //     //         .comment_delimiters("[#", "#]")
        //     //         .build()
        //     //         .unwrap(),
        //     // );
        //     // env.add_template("t", include_str!("../dev/template.neoj"))
        //     //     .unwrap();
        //     // match env.get_template("t") {
        //     //     Ok(template) => {
        //     //         match template.render(
        //     //             context!(page => Value::from_serialize(&sections)),
        //     //         ) {
        //     //             Ok(output) => {
        //     //                 let tmp_html_path =
        //     //                     PathBuf::from("dev-output/basic/index.html");
        //     //                 let _ = std::fs::write(tmp_html_path, output);
        //     //             }
        //     //             Err(e) => {
        //     //                 dbg!(e);
        //     //                 ()
        //     //             }
        //     //         }
        //     //     }
        //     //     Err(e) => {
        //     //         dbg!(e);
        //     //         ()
        //     //     }
        //     // }
        //     // let tmp_json_path = PathBuf::from("dev-output/basic/ast.json");
        //     // let json = serde_json::to_string_pretty(&sections).unwrap();
        //     // std::fs::write(tmp_json_path, json);
        // } else {
        //     assert!(false);
        // }
    }

    //
}
