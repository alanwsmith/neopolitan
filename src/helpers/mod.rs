#![allow(unused)]
use crate::block::Block;
use crate::block_metadata::parent::BlockParent;
use crate::config::Config;
use crate::span::Span;
use anyhow::Result;
use nom::IResult;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

pub enum TestCase {
    Ok {
        description: String,
        json: String,
        path: String,
        remainder: String,
        source: String,
    },
    // TODO: I think Err is the same as ExpectingErr
    Err {
        description: String,
        path: String,
        source: String,
    },
    ExpectingErr {
        description: String,
        path: String,
        source: String,
    },
    Skip,
}

pub enum TestBlockPayload {
    Ok {
        left_content: (String, Block),
        right_content: (String, Block),
        left_remainder: (String, String),
        right_remainder: (String, String),
    },
    Skip,
    ExpectedError,
    ShouldHaveErroredButDidNot,
    UnexpectedError,
}

pub enum TestSpanPayload {
    Ok {
        left_content: (String, Span),
        right_content: (String, Span),
        left_remainder: (String, String),
        right_remainder: (String, String),
    },
    Skip,
    ExpectedError,
    ShouldHaveErroredButDidNot,
}

pub fn get_file_list(
    source_dir: &PathBuf,
    extensions: &Vec<String>,
) -> Result<Vec<PathBuf>> {
    let walker = WalkDir::new(source_dir).into_iter();
    let files = walker
        .filter_map(|path_result| match path_result {
            Ok(path) => match path.path().extension() {
                Some(ext) => {
                    if extensions.contains(&ext.display().to_string()) {
                        Some(path.path().to_path_buf())
                    } else {
                        None
                    }
                }
                None => None,
            },
            Err(_) => None,
        })
        .collect();
    Ok(files)
}

pub fn get_test_data(source_path: &PathBuf) -> TestCase {
    let content = fs::read_to_string(source_path).unwrap();
    let parts: Vec<_> = content
        .split("~~~~~~")
        .filter_map(|part| Some(part.trim_end().to_string()))
        .collect();
    let source = match parts[1].as_str() {
        "json" => {
            let json: Value = serde_json::from_str(&parts[0]).unwrap();
            json.get("content").unwrap().as_str().unwrap().to_string()
        }
        _ => parts[0].clone(),
    };
    if parts[2] == "skip" {
        TestCase::Skip
    } else if parts[2] == "error" {
        TestCase::ExpectingErr {
            description: parts[3].clone(),
            path: source_path.display().to_string(),
            source,
        }
    } else {
        if parts.len() == 6 {
            let remainder_json: Value =
                serde_json::from_str(&parts[5]).unwrap();
            TestCase::Ok {
                path: source_path.display().to_string(),
                description: parts[1].clone(),
                source,
                json: parts[4].clone(),
                remainder: remainder_json
                    .get("remainder")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
            }
        } else {
            TestCase::Err {
                path: source_path.display().to_string(),
                description: parts[1].clone(),
                source,
            }
        }
    }
}

pub fn run_block_test_case_with_source_config_parent(
    source_path: &PathBuf,
    config: &Config,
    parent: &BlockParent,
    f: &dyn for<'a> Fn(
        &'a str,
        &'a Config,
        &'a BlockParent,
    ) -> IResult<&'a str, Block>,
) -> TestBlockPayload {
    match get_test_data(&source_path) {
        TestCase::Skip => TestBlockPayload::Skip,
        TestCase::Ok {
            json,
            path,
            remainder,
            source,
            ..
        } => {
            let result = f(&source, config, parent).unwrap();
            let left_content = (
                format!("Content: {}", &path),
                serde_json::from_str::<Block>(&json).unwrap(),
            );
            let right_content = (format!("Content: {}", &path), result.1);
            let left_remainder = (format!("Remainder: {}", &path), remainder);
            let right_remainder =
                (format!("Remainder: {}", &path), result.0.to_string());
            TestBlockPayload::Ok {
                left_content,
                right_content,
                left_remainder,
                right_remainder,
            }
        }
        _ => TestBlockPayload::Skip,
    }
}

pub fn run_block_test_case_with_source_config_parent_parent_kind(
    source_path: &PathBuf,
    config: &Config,
    parent: &BlockParent,
    parent_kind: &str,
    f: &dyn for<'a> Fn(
        &'a str,
        &'a Config,
        &'a BlockParent,
        &'a str,
    ) -> IResult<&'a str, Block>,
) -> TestBlockPayload {
    match get_test_data(&source_path) {
        TestCase::ExpectingErr {
            description,
            path,
            source,
        } => {
            if f(&source, config, parent, parent_kind).is_err() {
                TestBlockPayload::ExpectedError
            } else {
                TestBlockPayload::ShouldHaveErroredButDidNot
            }
        }

        TestCase::Skip => TestBlockPayload::Skip,
        TestCase::Ok {
            json,
            path,
            remainder,
            source,
            ..
        } => {
            let result = f(&source, config, parent, parent_kind).unwrap();
            let left_content = (
                format!("Content: {}", &path),
                serde_json::from_str::<Block>(&json).unwrap(),
            );
            let right_content = (format!("Content: {}", &path), result.1);
            let left_remainder = (format!("Remainder: {}", &path), remainder);
            let right_remainder =
                (format!("Remainder: {}", &path), result.0.to_string());
            TestBlockPayload::Ok {
                left_content,
                right_content,
                left_remainder,
                right_remainder,
            }
        }
        TestCase::Err { .. } => {
            dbg!("TODO: Get better messaging and handling here if needed");
            TestBlockPayload::UnexpectedError
        }
    }
}

pub fn run_span_test_case(
    source_path: &PathBuf,
    f: &dyn Fn(&str) -> IResult<&str, Span>,
) -> TestSpanPayload {
    match get_test_data(&source_path) {
        TestCase::Skip => TestSpanPayload::Skip,
        TestCase::ExpectingErr {
            description,
            path,
            source,
        } => {
            if f(&source).is_err() {
                TestSpanPayload::ExpectedError
            } else {
                TestSpanPayload::ShouldHaveErroredButDidNot
            }
        }
        TestCase::Ok {
            json,
            path,
            remainder,
            source,
            ..
        } => {
            let result = f(&source).unwrap();
            let left_content = (
                format!("Content: {}", &path),
                serde_json::from_str::<Span>(&json).unwrap(),
            );
            let right_content = (format!("Content: {}", &path), result.1);
            let left_remainder = (format!("Remainder: {}", &path), remainder);
            let right_remainder =
                (format!("Remainder: {}", &path), result.0.to_string());
            TestSpanPayload::Ok {
                left_content,
                right_content,
                left_remainder,
                right_remainder,
            }
        }
        _ => TestSpanPayload::Skip,
    }
}
