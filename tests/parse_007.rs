#![allow(unused_imports)]
#[cfg(test)]
mod tests {

    use neopolitan::page::Page;
    use neopolitan::parse_dev::parse_dev;
    use neopolitan::parse_switch::parse_switch;
    use neopolitan::section::Section;
    use std::collections::HashMap;

    #[test]
    fn test_004() {
        let source = r#"
-> LIST 

- alfa bravo

- charlie delta 

"#;

        // NOTE that Rust has a bunch of whitespace after it
        // for the test

        let page = Page {
            attributes: HashMap::new(),
            children: vec![Section::UNORDERED_LIST {
                attributes: HashMap::new(),
                children: vec![
                    Section::UNORDERED_LIST_ITEM {
                        attributes: HashMap::new(),
                        children: vec![Section::P {
                            attributes: HashMap::new(),
                            children: vec![Section::PLAINTEXT {
                                value: "alfa bravo".to_string(),
                            }],
                        }],
                    },
                    Section::UNORDERED_LIST_ITEM {
                        attributes: HashMap::new(),
                        children: vec![Section::P {
                            attributes: HashMap::new(),
                            children: vec![Section::PLAINTEXT {
                                value: "charlie delta".to_string(),
                            }],
                        }],
                    },
                ],
            }],
            categories: vec![],
        };

        let expected = page;
        let result = parse_dev(source);
        assert_eq!(expected, result);
    }
}
