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
    Err {
        description: String,
        path: String,
        source: String,
    },
    Skip,
}

pub enum TestSpanPayload {
    Ok {
        left_content: (String, Span),
        right_content: (String, Span),
        left_remainder: (String, String),
        right_remainder: (String, String),
    },
    Skip,
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

//pub fn get_test_payload_span(
//    test_dir: &PathBuf,
//    f: &dyn Fn(&str) -> IResult<&'static str, Span>,
//) -> TestSpanPayload {
//    TestSpanPayload::Ok {
//        left: ("asdf".to_string(), "wer".to_string()),
//        right: ("asdf".to_string(), "wer".to_string()),
//    }
//    // let config = Config::default();
//    // let file_list = get_file_list(
//    //     &PathBuf::from("src/block/list_item_spans/tests"),
//    //     &vec!["neotest".to_string()],
//    // )
//    // .unwrap();
//    // for source_path in file_list {
//    //     match get_test_data(&source_path) {
//    //         TestCase::Skip => {
//    //             assert!(true);
//    //         }
//    //         TestCase::Ok {
//    //             description,
//    //             json,
//    //             path,
//    //             remainder,
//    //             source,
//    //         } => {
//    //             println!("test {}", &path);
//    //             let result = list_item_spans(
//    //                 &source,
//    //                 &config,
//    //                 &BlockParent::ListItem,
//    //                 "list-item",
//    //             )
//    //             .unwrap();
//    //             let left_content = (
//    //                 format!("Content: {}", &path),
//    //                 serde_json::from_str::<Block>(&json).unwrap(),
//    //             );
//    //             let right_content = (format!("Content: {}", &path), result.1);
//    //             assert_eq!(left_content, right_content);
//    //             let left_remainder =
//    //                 (format!("Remainder: {}", &path), remainder);
//    //             let right_remainder =
//    //                 (format!("Remainder: {}", &path), result.0.to_string());
//    //             assert_eq!(left_remainder, right_remainder);
//    //         }
//    //         TestCase::Err {
//    //             description,
//    //             path,
//    //             source,
//    //         } => {
//    //             println!("test {}", &path);
//    //             let result = list_item_spans(
//    //                 &source,
//    //                 &config,
//    //                 &BlockParent::ListItem,
//    //                 "list-item",
//    //             );
//    //             match result {
//    //                 Ok(_) => {
//    //                     println!(
//    //                         "ERROR: Should not have gotten valid response"
//    //                     );
//    //                     assert!(false);
//    //                 }
//    //                 Err(_) => {
//    //                     assert!(true);
//    //                 }
//    //             }
//    //         }
//    //     }
//    // }
//    //
//}

pub fn run_span_test_case(
    source_path: &PathBuf,
    f: &dyn Fn(&str) -> IResult<&str, Span>,
) -> TestSpanPayload {
    // let config = Config::default();
    match get_test_data(&source_path) {
        TestCase::Skip => TestSpanPayload::Skip,
        TestCase::Ok {
            json,
            path,
            remainder,
            source,
            ..
        } => {
            println!("test {}", &path);
            let result = f(&source).unwrap();
            let left_content = (
                format!("Content: {}", &path),
                serde_json::from_str::<Span>(&json).unwrap(),
            );
            let right_content = (format!("Content: {}", &path), result.1);

            //assert_eq!(left_content, right_content);
            let left_remainder = (format!("Remainder: {}", &path), remainder);
            let right_remainder =
                (format!("Remainder: {}", &path), result.0.to_string());
            // assert_eq!(left_remainder, right_remainder);
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
