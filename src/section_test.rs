use crate::chunk::Chunk;
use crate::section::section;
use crate::section::Section;
use std::collections::HashMap;

#[test]
fn test_get_structure_001() {
    let source = r#"-> TITLE

Alfa Bravo
"#;
    let expected = Section::TITLE {
        children: vec![Chunk::H1 {
            attributes: HashMap::new(),
            children: vec![Chunk::Text {
                value: "Alfa Bravo".to_string(),
            }],
        }],
    };
    let result = section(source).unwrap().1;
    assert_eq!(expected, result);
}
