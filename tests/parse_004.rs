#![allow(unused_imports)]
#[cfg(test)]
mod tests {

    use neopolitan::parse_switch::parse_switch;
    // use neopolitan::content::Content;
    use neopolitan::page::Page;
    use neopolitan::parse_dev::parse_dev;
    // use neopolitan::parse_switch::parse_switch;
    // use neopolitan::section::Section;
    use std::collections::HashMap;

    #[test]
    fn test_004() {
        let source = r#"
-> ATTRIBUTES 
-> date: 2023-03-03 04:05:06
"#;

        let mut page = Page {
            attributes: HashMap::new(),
            children: vec![],
            categories: vec![],
        };
        page.attributes
            .insert("date".to_string(), "2023-03-03 04:05:06".to_string());
        let expected = page;
        let result = parse_switch(source);
        assert_eq!(expected, result);
    }
}
