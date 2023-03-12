use crate::parse::get_text;
use crate::parse::Section;
use nom::IResult;
use std::collections::HashMap;

pub fn p(source: &str) -> IResult<&str, Section> {
    Ok((
        "",
        Section::P {
            attributes: HashMap::new(),
            children: vec![get_text(source).unwrap().1],
        },
    ))
}
