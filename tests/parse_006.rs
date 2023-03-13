#![allow(unused_imports)]
#[cfg(test)]
mod tests {

    use neopolitan::page::Page;
    use neopolitan::parse_dev::parse_dev;
    use neopolitan::parse_switch::parse_switch;
    use std::collections::HashMap;

    #[test]
    fn test_004() {
        let source = r#"
-> CATEGORIES 

- Rust
- Test
"#;

        let page = Page {
            attributes: HashMap::new(),
            children: vec![],
            categories: vec!["Rust".to_string(), "Test".to_string()],
        };
        let expected = page;
        let result = parse_dev(source);
        assert_eq!(expected, result);
    }
}
