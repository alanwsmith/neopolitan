use crate::ast::Ast;
use crate::config::Config;
use crate::section::Section;
use anyhow::Result;
use minijinja::syntax::SyntaxConfig;
use minijinja::{Environment, Value, context, path_loader};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;
use walkdir::WalkDir;

// NOTE: this creates an Environment three times
// could be optimized to just one, but it's fine
// for now

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Site {
    config: Config,
    errors: BTreeMap<PathBuf, (String, String)>,
    files: Vec<PathBuf>,
    incompletes: BTreeMap<PathBuf, (Vec<Section>, String)>,
    input_root: PathBuf,
    output_root: PathBuf,
    pages: BTreeMap<PathBuf, Vec<Section>>,
}

impl Site {
    pub fn load_pages_and_files(&mut self) -> Result<()> {
        let input_files = get_files_in_dir(&self.input_root)?;
        input_files.iter().for_each(|source_path| {
            let source_extension = source_path.extension().unwrap();
            if source_extension == "neo" {
                let source =
                    std::fs::read_to_string(&source_path).unwrap().to_string();
                match Ast::new_from_source(&source, &self.config, false) {
                    Ast::Ok { sections } => {
                        self.pages
                            .insert(source_path.clone(), sections.clone());
                    }
                    Ast::Error { message, remainder } => {
                        self.errors
                            .insert(source_path.clone(), (message, remainder));
                    }
                    Ast::Incomplete { parsed, remainder } => {
                        self.incompletes.insert(
                            source_path.clone(),
                            (parsed, remainder.to_string()),
                        );
                    }
                }
            } else {
                self.files.push(source_path.clone());
            }
        });
        Ok(())
    }

    pub fn copy_files(&self) -> Result<()> {
        self.files.iter().for_each(|source_path| {
            let output_path = replace_path_root(
                &source_path,
                &self.input_root,
                &self.output_root,
            )
            .unwrap();
            let parent = output_path.parent().unwrap();
            std::fs::create_dir_all(parent).unwrap();
            std::fs::copy(source_path, output_path).unwrap();
        });
        Ok(())
    }

    pub fn output_pages(&self) -> Result<()> {
        let site = &self.clone();
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
        self.pages.iter().for_each(|(source_path, sections)| {
            let mut output_path = replace_path_root(
                &source_path,
                &self.input_root,
                &self.output_root,
            )
            .unwrap();
            output_path.set_extension("html");
            dbg!(&output_path);
            let parent = output_path.parent().unwrap();
            std::fs::create_dir_all(parent).unwrap();
            let template =
                env.get_template("pages/template-picker.neoj").unwrap();
            let sections = Value::from_serialize(&sections);
            match template.render(context!(sections)) {
                Ok(output) => {
                    write_file_with_mkdir(&output_path, &output).unwrap()
                }
                Err(e) => {
                    // Attempt to fall back to error output
                    let output_error_template =
                        env.get_template("pages/rendering-error.neoj").unwrap();
                    let name = Value::from(e.name().unwrap());
                    let detail = Value::from(e.detail().unwrap());
                    let line = Value::from(e.line().unwrap());
                    let debug = Value::from(e.display_debug_info().to_string());
                    let context = context!(site, debug, detail, line, name);
                    match output_error_template.render(context) {
                        Ok(error_output) => {
                            write_file_with_mkdir(&output_path, &error_output)
                                .unwrap()
                        }
                        // Panic on purpose if the error file
                        // can't be written
                        Err(panic_error) => {
                            dbg!(panic_error);
                            assert!(false);
                        }
                    }
                }
            }
        });
        Ok(())
    }

    pub fn output_errors(&self) -> Result<()> {
        let site = &self.clone();
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
        self.errors
            .iter()
            .for_each(|(source_path, (message, remainder))| {
                let mut output_path = replace_path_root(
                    &source_path,
                    &self.input_root,
                    &self.output_root,
                )
                .unwrap();
                output_path.set_extension("html");
                dbg!(&output_path);
                let parent = output_path.parent().unwrap();
                std::fs::create_dir_all(parent).unwrap();
                let template =
                    env.get_template("pages/template-picker.neoj").unwrap();
                let output = template
                    .render(context!(site, message, remainder))
                    .unwrap();
                write_file_with_mkdir(&output_path, &output).unwrap();
            });
        Ok(())
    }

    pub fn output_incompletes(&self) -> Result<()> {
        Ok(())
    }

    //
    //                 Ast::Incomplete { parsed, remainder } => {
    //                     let template =
    //                         env.get_template("pages/incomplete.neoj").unwrap();
    //                     let parsed = Value::from_serialize(parsed);
    //                     let remainder = Value::from_serialize(remainder);
    //                     let output = template
    //                         .render(context!(
    //                             parsed => parsed,
    //                             remainder => remainder,
    //                         ))
    //                         .unwrap();
    //                     write_file_with_mkdir(&output_path, &output).unwrap();
    //                 }
    //             };
    //         }
    //     });
    //     Ok(())
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solo_build_site() {
        // This is a hack to attempt to built the
        // site before running the rest of the
        // tests. A more appropriate method will
        // be added at a later date.
        let mut site = Site {
            config: Config::default(),
            errors: BTreeMap::new(),
            incompletes: BTreeMap::new(),
            input_root: PathBuf::from("docs-content"),
            output_root: PathBuf::from("docs"),
            pages: BTreeMap::new(),
            files: vec![],
        };
        // These will panic intentionally if
        // they fail for now
        site.load_pages_and_files().unwrap();
        site.copy_files().unwrap();
        site.output_pages().unwrap();
        site.output_errors().unwrap();
        site.output_incompletes().unwrap();
    }
}
