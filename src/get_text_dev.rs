use crate::section::Section;
use nom::IResult;
use std::collections::HashMap;

pub fn get_text_dev(source: &str) -> IResult<&str, Vec<Section>> {
    let texts = vec![
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

    Ok((source, texts))
}
