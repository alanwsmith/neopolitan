// use crate::block::block::*;
use crate::section::section::*;
// use crate::snippet::snippet::*;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub fn parse(source: &str) -> IResult<&str, Option<Vec<Section>>> {
    let (_, sections) = many_till(section, eof)(source)?;
    Ok(("", Some(sections.0)))
}

pub fn parse_dev(source: &str) -> IResult<&str, Option<Vec<Section>>> {
    let (_, sections) = many_till(section, eof)(source)?;
    Ok(("", Some(sections.0)))

    // dbg!(sections);
    // Ok((
    //     "",
    //     Some(vec![Section::TitleSection {
    //         attributes: Some(vec![(Some("id".to_string()), Some("bravo".to_string()))]),
    //         children: Some(vec![Block::Text {
    //             snippets: Some(vec![Snippet::Plain {
    //                 text: Some("Set The Piece".to_string()),
    //             }]),
    //         }]),
    //     }]),
    // ))
}
