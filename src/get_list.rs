use crate::section::Section;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until1;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::IResult;
use std::collections::HashMap;

pub fn list_item_splitter(source: &str) -> IResult<&str, Section> {
    // dbg!(source);
    let (source, _) = multispace0(source)?;
    // dbg!(source);
    let (source, _) = tag("- ")(source)?;
    // dbg!(source);
    let (_, text) = alt((take_until1("\n\n"), rest))(source)?;
    // dbg!(source);

    let list_item = Section::UNORDERED_LIST_ITEM {
        attributes: HashMap::new(),
        children: vec![Section::P {
            attributes: HashMap::new(),
            children: vec![Section::PLAINTEXT {
                value: text.to_string(),
            }],
        }],
    };
    // dbg!(text);
    Ok(("", list_item))
}

pub fn get_list(source: &str) -> IResult<&str, Section> {
    let (_, list_items) = many_till(list_item_splitter, eof)(source)?;
    dbg!(&list_items);

    // let list = Section::UNORDERED_LIST {
    //     attributes: HashMap::new(),
    //     children: vec![Section::UNORDERED_LIST_ITEM {
    //         attributes: HashMap::new(),
    //         children: vec![Section::P {
    //             attributes: HashMap::new(),
    //             children: vec![Section::PLAINTEXT {
    //                 value: "alfa bravo".to_string(),
    //             }],
    //         }],
    //     }],
    // };

    let list = Section::UNORDERED_LIST {
        attributes: HashMap::new(),
        children: list_items.0,
    };

    Ok((source, list))
}
