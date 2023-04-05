use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Attribute {
    Basic {
        key: Option<String>,
        value: Option<String>,
    },
}

pub fn attribute(source: &str) -> IResult<&str, Attribute> {
    let (_, b) = separated_list0(tag(":"), is_not(":"))(source)?;
    if b.len() > 1 {
        Ok((
            "",
            Attribute::Basic {
                key: Some(b[0].trim().to_string()),
                value: Some(b[1].trim().to_string()),
            },
        ))
    } else {
        Ok((
            "",
            Attribute::Basic {
                key: Some(b[0].trim().to_string()),
                value: None,
            },
        ))
    }
}
