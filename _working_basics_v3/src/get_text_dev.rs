use crate::section::Section;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;
use std::collections::HashMap;

pub fn split_text(source: &str) -> IResult<&str, Section> {
    dbg!(source);
    let text = Section::PLACEHOLDER;
    Ok(("", text))
}

pub fn get_text_dev(source: &str) -> IResult<&str, Vec<Section>> {
    let (source, _texts) = many_till(split_text, eof)(source)?;
    // Ok((source, paragraphs.0))
    //

    dbg!(source);
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
