use nom::IResult;

pub mod class;

#[derive(Debug, PartialEq)]
pub enum TagAttr {
    Class(Vec<String>),
    Placeholder
}

pub fn tag_attrs(source: &str) -> IResult<&str, Vec<TagAttr>> {
Ok((source, vec![]))
}