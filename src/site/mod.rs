use crate::ast::Ast;
use crate::block::Block;
use crate::config::Config;
use crate::minijinja_functions::highlight_syntax::highlight_span;
use crate::page::Page;
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
pub struct Site<'a> {
    pub config: Config,
    pub errors: BTreeMap<PathBuf, (String, String)>,
    pub files: Vec<(PathBuf, PathBuf)>,
    pub incompletes: BTreeMap<PathBuf, (Vec<Block>, String)>,
    pub input_root: PathBuf,
    pub output_root: PathBuf,
    pub pages: BTreeMap<PathBuf, Vec<Block>>,
    pub pages_dev: BTreeMap<PathBuf, Page<'a>>,
}

impl Site<'_> {
    pub fn load_pages_and_files(&mut self) -> Result<()> {
        let input_files = get_files_in_dir(&self.input_root)?;
        input_files.iter().for_each(|source_path| {
            let mut output_path = replace_path_root(
                &source_path,
                &self.input_root,
                &self.output_root,
            )
            .unwrap();
            let parent = output_path.parent().unwrap();
            std::fs::create_dir_all(parent).unwrap();
            let source_extension = source_path.extension().unwrap();
            if source_extension == "neo" {
                output_path.set_extension("html");
                let stripped_output_path = output_path
                    .strip_prefix(&self.output_root)
                    .unwrap()
                    .to_path_buf();
                let source =
                    std::fs::read_to_string(&source_path).unwrap().to_string();
                match Ast::new_from_source(&source, &self.config, false) {
                    Ast::Ok { blocks } => {
                        self.pages.insert(
                            stripped_output_path.clone(),
                            blocks.clone(),
                        );
                    }
                    Ast::Error { message, remainder } => {
                        self.errors.insert(
                            stripped_output_path.clone(),
                            (message, remainder),
                        );
                    }
                    Ast::Incomplete { parsed, remainder } => {
                        self.incompletes.insert(
                            stripped_output_path.clone(),
                            (parsed, remainder.to_string()),
                        );
                    }
                }
            } else {
                self.files.push((source_path.clone(), output_path.clone()));
            }
        });
        Ok(())
    }

    pub fn copy_files(&self) -> Result<()> {
        self.files.iter().for_each(|(source_path, output_path)| {
            std::fs::copy(source_path, output_path).unwrap();
        });
        Ok(())
    }

    pub fn output_pages(&self) -> Result<()> {
        let site = Value::from_serialize(&self.clone());
        let mut env = Environment::new();
        env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );
        env.add_function("highlight_span", highlight_span);
        env.set_loader(path_loader("docs-content/reference-templates"));
        // env.set_loader(path_loader("docs-templates"));
        self.pages.iter().for_each(|(relative_path, blocks)| {
            let output_path = &self.output_root.join(relative_path);
            let template =
                env.get_template("helpers/template-picker.neoj").unwrap();
            let blocks = Value::from_serialize(&blocks);
            match template.render(context!(site, blocks)) {
                Ok(output) => {
                    write_file_with_mkdir(&output_path, &output).unwrap()
                }
                Err(e) => {
                    // Attempt to fall back to error output
                    let output_error_template = env
                        .get_template("helpers/rendering-error.neoj")
                        .unwrap();
                    let name = Value::from(e.name().unwrap());
                    let message = Value::from(e.detail().unwrap());
                    let line = Value::from(e.line().unwrap());
                    let mut e = &e as &dyn std::error::Error;
                    let mut error_details = vec![];
                    while let Some(next_err) = e.source() {
                        error_details.push(format!("{:#}", next_err));
                        e = next_err;
                    }
                    error_details.reverse();
                    let details = Value::from_serialize(error_details.clone());
                    let context = context!(site, details, line, message, name);
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
        let site = Value::from_serialize(&self.clone());
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
            .for_each(|(relative_path, (message, remainder))| {
                let output_path = &self.output_root.join(relative_path);
                let template =
                    env.get_template("pages/parsing-error.neoj").unwrap();
                let output = template
                    .render(context!(site, message, remainder))
                    .unwrap();
                write_file_with_mkdir(&output_path, &output).unwrap();
            });
        Ok(())
    }

    pub fn output_incompletes(&self) -> Result<()> {
        let site = Value::from_serialize(&self.clone());
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
        self.incompletes
            .iter()
            .for_each(|(relative_path, (sections, remainder))| {
                let output_path = &self.output_root.join(relative_path);
                let template =
                    env.get_template("helpers/incomplete.neoj").unwrap();
                let output = template
                    .render(context!(site, sections => Value::from_serialize(sections), remainder))
                    .unwrap();
                write_file_with_mkdir(&output_path, &output).unwrap();
            });
        Ok(())
    }
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
