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
    Ok {
        sections: Vec<Section>,
    },
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
                    Ast::Ok {
                        sections: results.1,
                    }
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
    use anyhow::Result;
    use minijinja::syntax::SyntaxConfig;
    use minijinja::{Environment, Value, context, path_loader};
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;
    use walkdir::WalkDir;
    // use serde_json;

    #[test]
    fn basic_test() {
        let config = Config::default();
        let source = include_str!("test-data/basic-example.neo");
        if let Ast::Ok { sections } =
            Ast::new_from_source(source, &config, false)
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
        if let Ast::Ok { sections } =
            Ast::new_from_source(source, &config, false)
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

    // DEPRECATED: Remove the next time you see it
    // #[test]
    // fn test_spec_output() {
    //     let mut env = Environment::new();
    //     env.set_syntax(
    //         SyntaxConfig::builder()
    //             .block_delimiters("[!", "!]")
    //             .variable_delimiters("[@", "@]")
    //             .comment_delimiters("[#", "#]")
    //             .build()
    //             .unwrap(),
    //     );
    //     env.set_loader(path_loader("src/templates/spec"));
    // }

    #[test]
    fn solo_output_site() -> Result<()> {
        let config = Config::default();
        let mut env = Environment::new();
        env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );
        env.set_loader(path_loader("docs-content/reference-templates"));
        // env.set_loader(path_loader("docs-templates"));
        let input_files = get_files_in_dir(&PathBuf::from("docs-content"))?;
        input_files.iter().for_each(|source_path| {
            let source = std::fs::read_to_string(&source_path).unwrap();
            let source_extension = source_path.extension().unwrap();
            let mut output_path = replace_path_root(
                &source_path,
                &PathBuf::from("docs-content"),
                &PathBuf::from("docs"),
            )
            .unwrap();
            if source_extension == "css" || source_extension == "html" || source_extension == "neoj" {
                // This panics if it can't make the directory 
                // or copy the file. Should be fine for dev.
                let parent = output_path.parent().unwrap();
                std::fs::create_dir_all(parent).unwrap();
                std::fs::copy(source_path, output_path).unwrap();
            } else {
                output_path.set_extension("html");
                // NOTE: these will panic intentionally if they
                // can't be output
                match Ast::new_from_source(&source, &config, false) {
                    Ast::Ok { sections } => {
                        let template = env.get_template("pages/template-picker.neoj").unwrap();
                        let sections = Value::from_serialize(&sections);
                        match template.render(context!(sections => sections)) {
                            Ok(output) => {
                                write_file_with_mkdir(&output_path, &output)
                                    .unwrap()
                            }
                            Err(e) => {
                                // Attempt to fall back to error output
                                let output_error_template = env
                                    .get_template("pages/rendering-error.neoj")
                                    .unwrap();
                                let name = Value::from(e.name().unwrap());
                                let detail = Value::from(e.detail().unwrap());
                                let line = Value::from(e.line().unwrap());
                                let debug = Value::from(
                                    e.display_debug_info().to_string(),
                                );
                                match output_error_template.render(context!(debug => debug, detail => detail, line => line, name=> name)) {
                                    Ok(error_output) => write_file_with_mkdir(
                                        &output_path,
                                        &error_output,
                                    )
                                    .unwrap(),
                                    // Panic on purpose if the error file
                                    // can't be written
                                    Err(panic_error) => {
                                        dbg!(panic_error);
                                        assert!(false);
                                    }
                                }
                            }
                        }
                    }
                    Ast::Error { message, remainder } => {
                        let template = env.get_template("pages/parsing-error.neoj").unwrap();
                        let message = Value::from_serialize(message);
                        let remainder = Value::from_serialize(remainder);
                        let output = template
                            .render(context!(
                                message => message,
                                remainder => remainder,
                            ))
                            .unwrap();
                        write_file_with_mkdir(&output_path, &output).unwrap();
                    }
                    Ast::Incomplete { parsed, remainder } => {
                        let template =
                            env.get_template("pages/incomplete.neoj").unwrap();
                        let parsed = Value::from_serialize(parsed);
                        let remainder = Value::from_serialize(remainder);
                        let output = template
                            .render(context!(
                                parsed => parsed,
                                remainder => remainder,
                            ))
                            .unwrap();
                        write_file_with_mkdir(&output_path, &output).unwrap();
                    }
                };
            }
            //output_page(&source_path, &output_path);
        });
        // dbg!(output_files);
        //        output_page(&input, &output);
        // This isn't really a test. It's what's
        // used to generate the https://neopolitan.alanwsmith.com/
        // site with the demo templates. This
        // serves as way to validate output
        // during development each time the
        // tests are run.
        // assert!(false);
        Ok(())
    }

    // This is a support function for output_site
    fn output_page(input: &PathBuf, output: &PathBuf) {
        // let config = Config::default();
        // let source = include_str!("test-data/output-example.neo");
        // if let Ast::Ok(sections) = Ast::new_from_source(source, &config, false)
        // {
        //     let mut env = Environment::new();
        //     env.set_syntax(
        //         SyntaxConfig::builder()
        //             .block_delimiters("[!", "!]")
        //             .variable_delimiters("[@", "@]")
        //             .comment_delimiters("[#", "#]")
        //             .build()
        //             .unwrap(),
        //     );
        //     env.add_template(
        //         "page.neoj",
        //         include_str!("../dev-templates/page.neoj"),
        //     )
        //     .unwrap();
        //     env.add_template(
        //         "basic.neoj",
        //         include_str!("../dev-templates/basic.neoj"),
        //     )
        //     .unwrap();
        //     env.add_template(
        //         "block.neoj",
        //         include_str!("../dev-templates/block.neoj"),
        //     )
        //     .unwrap();
        //     env.add_template(
        //         "spans/code-span.neoj",
        //         include_str!("../dev-templates/spans/code-span.neoj"),
        //     )
        //     .unwrap();
        //     env.add_template(
        //         "spans/text-span.neoj",
        //         include_str!("../dev-templates/spans/text-span.neoj"),
        //     )
        //     .unwrap();
        //     env.add_template(
        //         "macros.neoj",
        //         include_str!("../dev-templates/macros.neoj"),
        //     )
        //     .unwrap();
        //     match env.get_template("page.neoj") {
        //         Ok(template) => {
        //             match template.render(
        //                 context!(page => Value::from_serialize(&sections)),
        //             ) {
        //                 Ok(output) => {
        //                     let tmp_html_path =
        //                         PathBuf::from("dev-output/basic/index.html");
        //                     let _ = std::fs::write(tmp_html_path, output);
        //                 }
        //                 Err(e) => {
        //                     dbg!(e);
        //                     ()
        //                 }
        //             }
        //         }
        //         Err(e) => {
        //             dbg!(e);
        //             ()
        //         }
        //     }
        //     // let tmp_json_path = PathBuf::from("dev-output/basic/ast.json");
        //     // let json = serde_json::to_string_pretty(&sections).unwrap();
        //     // std::fs::write(tmp_json_path, json);
        //     // assert_eq!(1, sections.len());
        // } else {
        //     assert!(false);
        // }

        //
    }

    pub fn get_files_in_dir(root_dir: &PathBuf) -> Result<Vec<PathBuf>> {
        let file_list: Vec<PathBuf> = WalkDir::new(root_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .filter(|e| {
                !e.file_name()
                    .to_str()
                    .map(|s| s.starts_with("."))
                    .unwrap_or(false)
            })
            .map(|e| e.path().to_path_buf())
            .collect();
        Ok(file_list)
    }

    pub fn replace_path_root(
        source: &PathBuf,
        find: &PathBuf,
        replacement: &PathBuf,
    ) -> Result<PathBuf> {
        let stripped_path = source.strip_prefix(find)?;
        let new_path = replacement.clone().join(stripped_path);
        Ok(new_path)
    }

    fn write_file_with_mkdir(path: &PathBuf, content: &str) -> Result<()> {
        let parent_dir = path
            .parent()
            .ok_or(std::io::Error::other("Could not get parent path"))?;
        std::fs::create_dir_all(parent_dir)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
