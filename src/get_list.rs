use crate::section::Section;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until1;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;
use std::collections::HashMap;

pub fn list_item_splitter(source: &str) -> IResult<&str, Section> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("- ")(source)?;
    let (source, text) = take_until1("\n\n")(source)?;
    dbg!(source);
    dbg!(text);
    Ok(("", Section::PLACEHOLDER))
}

pub fn get_list(source: &str) -> IResult<&str, Section> {
    let (_, _list_items) = many_till(list_item_splitter, eof)(source)?;

    let list = Section::UNORDERED_LIST {
        attributes: HashMap::new(),
        children: vec![
            Section::UNORDERED_LIST_ITEM {
                attributes: HashMap::new(),
                children: vec![Section::P {
                    attributes: HashMap::new(),
                    children: vec![Section::PLAINTEXT {
                        value: "alfa bravo charlie delta".to_string(),
                    }],
                }],
            },
            Section::UNORDERED_LIST_ITEM {
                attributes: HashMap::new(),
                children: vec![Section::P {
                    attributes: HashMap::new(),
                    children: vec![Section::PLAINTEXT {
                        value: "echo foxtrot".to_string(),
                    }],
                }],
            },
        ],
    };

    Ok((source, list))
}
