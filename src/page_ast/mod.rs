use crate::block::Block;
use crate::block::block;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use nom::IResult;
use nom::Parser;
use nom::character::complete::multispace0;
use nom::multi::many1;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PageAst {
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

impl<'a> PageAst {
    pub fn new_from_source(source: &'a str, config: &'a Config) -> PageAst {
        match PageAst::parse_ast(source, config, &BlockParent::Page) {
            Ok(results) => {
                if results.0.is_empty() {
                    PageAst::Ok { blocks: results.1 }
                } else {
                    PageAst::Incomplete {
                        parsed: results.1,
                        remainder: results.0.to_string(),
                    }
                }
            }
            Err(_e) => PageAst::Error {
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
    use include_dir::{Dir, include_dir};
    use pretty_assertions::assert_eq;
    use serde_json;

    static TESTS_DIR: Dir<'_> = include_dir!("src/page_ast/tests");

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub struct TestObject {
        data: PageAst,
    }


    #[test]
    fn solo_ast_integration_tests() {
        let config = Config::default();
        let glob = "**/*.neo";
        for entry in TESTS_DIR.find(glob).unwrap() {
            if let Some(file) = entry.as_file() {
                if let Some(contents) = file.contents_utf8() {
                    dbg!(entry.path().display());
                    let parts = contents
                        .split("-- json")
                        .map(|part| part.to_string())
                        .collect::<Vec<String>>();
                    let under_test = TestObject {
                        data: PageAst::new_from_source(&parts[0], &config),
                    };
                    let target: TestObject =
                        serde_json::from_str(&parts[1]).unwrap();
                    if under_test != target {
                        let _ = std::fs::write(
                            "/Users/alan/Desktop/neopolitan-tmp.json",
                            serde_json::to_string_pretty(&under_test).unwrap(),
                        );
                    }
                    assert_eq!(under_test, target);
                }
            }
        }
    }

    //
}
