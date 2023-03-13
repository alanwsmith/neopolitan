#![allow(unused_imports)]
#[cfg(test)]
mod tests {
    use neopolitan::content::Content;
    use neopolitan::page::Page;
    use neopolitan::parse_dev::parse_dev;
    use neopolitan::parse_switch::parse_switch;
    use neopolitan::section::Section;
    use std::collections::HashMap;

    #[test]
    fn test_003() {
        let source = r#"
-> BLURB 

This is the blurb

"#;

        let mut page = Page {
            attributes: HashMap::new(),
            children: vec![],
        };
        page.attributes
            .insert("blurb".to_string(), "This is the blurb".to_string());
        let expected = page;
        let result = parse_switch(source);
        assert_eq!(expected, result);
    }
}
