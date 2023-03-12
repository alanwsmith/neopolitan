#![allow(unused_imports)]
#[cfg(test)]
mod tests {
    use neopolitan::content::Content;
    use neopolitan::page::Page;
    use neopolitan::parse::*;
    use neopolitan::parse_dev::*;
    use neopolitan::section::Section;
    use std::collections::HashMap;

    #[test]
    fn test_001() {
        let source = r#"
-> TITLE

This Is A Title

"#;
        let expected = Page {
            attributes: HashMap::new(),
            children: vec![Section::TITLE {
                attributes: HashMap::new(),
                children: vec![Content::PlainText {
                    value: "This Is A Title".to_string(),
                }],
            }],
        };
        let result = parse_dev(source);
        assert_eq!(expected, result);
    }

    // use crate::parse::parse;
    // use crate::parse::parse_dev;
    // use crate::parse::Page;
    // use std::fs;
    // use std::path::PathBuf;

    // #[test]
    // fn test_archive() {
    //     let tests: Vec<&str> = vec!["002.basic", "003.basic", "004.basic", "005.basic"];
    //     for test in tests {
    //         let mut source_path = "./test_targets/".to_string();
    //         source_path.push_str(test);
    //         source_path.push_str(".neo");
    //         let mut expected_path = "./test_targets/".to_string();
    //         expected_path.push_str(test);
    //         expected_path.push_str(".json");
    //         let source = fs::read_to_string(source_path).unwrap();
    //         let expected: Page =
    //             serde_json::from_str(fs::read_to_string(expected_path).unwrap().as_str()).unwrap();
    //         let result = parse(&source);
    //         assert_eq!(expected, result);
    //     }
    // }

    //     #[test]
    //     fn test_active() {
    //         let file_key = "006";
    //         let mut source_path = "./test_targets/".to_string();
    //         source_path.push_str(file_key);
    //         source_path.push_str(".neo");
    //         let mut expected_path = "./test_targets/".to_string();
    //         expected_path.push_str(file_key);
    //         expected_path.push_str(".json");
    //         let source = fs::read_to_string(source_path).unwrap();
    //         let expected: Page =
    //             serde_json::from_str(fs::read_to_string(expected_path).unwrap().as_str()).unwrap();
    //         let result = parse_dev(&source);
    //         // dbg!(&result);
    //         assert_eq!(expected, result);
    //     }
}
