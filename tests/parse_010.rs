#![allow(unused_imports)]
#[cfg(test)]
mod tests {

    use neopolitan::get_text::get_text;
    use neopolitan::page::Page;
    use neopolitan::parse_dev::parse_dev;
    use neopolitan::parse_switch::parse_switch;
    use neopolitan::section::Section;
    use std::collections::HashMap;

    #[test]
    fn test_010() {
        let source = r#"this is an <<link|example|https://www.example.com/>> link"#;
        let expected: Vec<Section> = vec![
            Section::PLAINTEXT {
                value: "this is an".to_string(),
            },
            Section::LINK {
                attributes: HashMap::new(),
                url: "https://www.example.com/".to_string(),
                value: "example".to_string(),
            },
            Section::PLAINTEXT {
                value: "link".to_string(),
            },
        ];
        let result = get_text(source).unwrap().1;
        // assert_eq!(expected, result);
    }
}
