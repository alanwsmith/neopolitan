use crate::chunk::Chunk;
use crate::get_structure::get_structure;
use crate::page::Page;
use crate::section::Section;
use std::collections::HashMap;

#[test]
fn test_get_structure_001() {
    let source = r#"-> TITLE

Alfa Bravo
"#;
    let expected = Page {
        attributes: HashMap::new(),
        children: vec![Section::TITLE {
            children: vec![Chunk::H1 {
                attributes: HashMap::new(),
                children: vec![Chunk::Text {
                    value: "Alfa Bravo".to_string(),
                }],
            }],
        }],
    };
    let result = get_structure(source).unwrap().1;
    assert_eq!(expected, result);
}
