#![allow(unused_imports)]
#[cfg(test)]
mod tests {

    use neopolitan::page::Page;
    use neopolitan::parse_dev::parse_dev;
    use neopolitan::parse_switch::parse_switch;
    use neopolitan::section::Section;
    use std::collections::HashMap;

    #[test]
    fn test_008() {
        let source = r#"
-> OLIST

- echo foxtrot

- golf hotel

"#;

        let page = Page {
            attributes: HashMap::new(),
            children: vec![Section::ORDERED_LIST {
                attributes: HashMap::new(),
                children: vec![
                    Section::ORDERED_LIST_ITEM {
                        attributes: HashMap::new(),
                        children: vec![Section::P {
                            attributes: HashMap::new(),
                            children: vec![Section::PLAINTEXT {
                                value: "echo foxtrot".to_string(),
                            }],
                        }],
                    },
                    Section::ORDERED_LIST_ITEM {
                        attributes: HashMap::new(),
                        children: vec![Section::P {
                            attributes: HashMap::new(),
                            children: vec![Section::PLAINTEXT {
                                value: "golf hotel".to_string(),
                            }],
                        }],
                    },
                ],
            }],
            categories: vec![],
        };

        let expected = page;
        let result = parse_switch(source);
        assert_eq!(expected, result);
    }
}
