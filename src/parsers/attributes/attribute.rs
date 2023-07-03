use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::sequence::preceded;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Attribute {
    Class(Vec<String>),
    Id(String),
    Empty,
}

pub fn attribute(source: &str) -> IResult<&str, Attribute> {
    dbg!(&source);
    let (source, attr_name) =
        preceded(tag("|"), is_not(":"))(source)?;
    match attr_name {
        "class" => Ok((
            source,
            Attribute::Class(vec![
                "alfa".to_string(),
                "bravo".to_string(),
            ]),
        )),
        _ => Ok((source, Attribute::Empty)),
    }
}
