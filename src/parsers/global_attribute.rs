pub mod class;

use crate::parsers::attribute::Attribute;
use nom::IResult;

pub fn global_attribute(source: &str) -> IResult<&str, Attribute> {
    Ok((source, Attribute::None))
}
