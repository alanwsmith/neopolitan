use crate::parse::get_text;
use crate::parse::Section;
use nom::IResult;
use std::collections::HashMap;

pub fn h1(source: &str) -> IResult<&str, Section> {
    Ok((
        "",
        Section::H1 {
            attributes: HashMap::new(),
            children: vec![get_text(source).unwrap().1],
        },
    ))
}

pub fn h2(source: &str) -> IResult<&str, Section> {
    Ok((
        "",
        Section::H2 {
            attributes: HashMap::new(),
            children: vec![get_text(source).unwrap().1],
        },
    ))
}
pub fn h3(source: &str) -> IResult<&str, Section> {
    Ok((
        "",
        Section::H3 {
            attributes: HashMap::new(),
            children: vec![get_text(source).unwrap().1],
        },
    ))
}
pub fn h4(source: &str) -> IResult<&str, Section> {
    Ok((
        "",
        Section::H4 {
            attributes: HashMap::new(),
            children: vec![get_text(source).unwrap().1],
        },
    ))
}
pub fn h5(source: &str) -> IResult<&str, Section> {
    Ok((
        "",
        Section::H5 {
            attributes: HashMap::new(),
            children: vec![get_text(source).unwrap().1],
        },
    ))
}

pub fn h6(source: &str) -> IResult<&str, Section> {
    Ok((
        "",
        Section::H6 {
            attributes: HashMap::new(),
            children: vec![get_text(source).unwrap().1],
        },
    ))
}
