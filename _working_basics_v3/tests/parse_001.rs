#![allow(unused_imports)]
#[cfg(test)]
mod tests {
    use neopolitan::page::Page;
    use neopolitan::parse_dev::parse_dev;
    use neopolitan::parse_switch::parse_switch;
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
                children: vec![Section::PLAINTEXT {
                    value: "This Is A Title".to_string(),
                }],
            }],
            categories: vec![],
        };
        let result = parse_switch(source);
        assert_eq!(expected, result);
    }
}
