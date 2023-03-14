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
    fn test_009() {
        let source = r#"kilo lima"#;
        let expected: Vec<Section> = vec![Section::PLAINTEXT {
            value: "kilo lima".to_string(),
        }];
        let result = get_text(source).unwrap().1;
        // assert_eq!(expected, result);
    }
}
