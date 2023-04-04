use crate::block::block::*;
use crate::section::section::*;
use crate::section::section_attributes::*;
use nom::bytes::complete::tag;
use nom::combinator::eof;
use nom::multi::many0;
use nom::multi::many_till;
use nom::sequence::preceded;
use nom::IResult;

pub fn p(source: &str) -> IResult<&str, Section> {
    dbg!(source);
    let (source, att_capture) = many0(preceded(tag(">> "), section_attribute))(source).unwrap();
    let attributes = if att_capture.is_empty() {
        None
    } else {
        Some(att_capture)
    };
    let (source, b) = many_till(block, eof)(source.trim()).unwrap();
    let children = if b.0.is_empty() { None } else { Some(b.0) };
    Ok((
        source,
        Section::Paragraphs {
            attributes,
            children,
        },
    ))
}

// pub fn title(source: &str) -> Section {
//     let (s, att_capture) = many0(preceded(tag(">> "), section_attribute))(source).unwrap();
//     let attributes = if att_capture.is_empty() {
//         None
//     } else {
//         Some(att_capture)
//     };
//     // still not sure this is the right way to go about this.
//     let (a, _) = multispace0::<&str, Error<&str>>(s).unwrap();
//     let (_, b) = many_till(block, eof)(a).unwrap();
//     let children = if b.0.is_empty() { None } else { Some(b.0) };
//     Section::Title {
//         attributes,
//         children,
//     }
// }
