#![allow(unused_variables)]

pub mod class_attr;

use crate::parsers::attribute::Attribute;
use nom::bytes::complete::is_not;
use nom::sequence::preceded;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::sequence::pair;
use nom::IResult;
use crate::parsers::global_attribute::class_attr::class_attr;

pub fn global_attribute(source: &str) -> IResult<&str, Attribute> {


    // let (source, attr_name) = preceded(tag("|"), is_not(":"))(source)?;
    // let (source, _) = pair(tag(":"), multispace0)(source)?;
    // dbg!(&attr_name);
    // dbg!(&source);

    // match attr_name {
    //     "class" => {
    //         class_attr(source)?;
    //         Ok((source, Attribute::Class(vec![])))
    //     },
    //     _ => {
    //         Ok((source, Attribute::None))
    //     }
    // }


    Ok((source, Attribute::None))
    
}
