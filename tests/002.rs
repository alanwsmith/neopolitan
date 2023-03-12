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

-> P 

First paragraph 

Second paragraph 

"#;
        let expected = Page {
            attributes: HashMap::new(),
            children: vec![
                Section::TITLE {
                    attributes: HashMap::new(),
                    children: vec![Content::PlainText {
                        value: "This Is A Title".to_string(),
                    }],
                },
                Section::P {
                    attributes: HashMap::new(),
                    children: vec![Content::PlainText {
                        value: "First paragraph".to_string(),
                    }],
                },
                Section::P {
                    attributes: HashMap::new(),
                    children: vec![Content::PlainText {
                        value: "Second paragraph".to_string(),
                    }],
                },
            ],
        };
        let result = parse_dev(source);
        assert_eq!(expected, result);
    }
}
