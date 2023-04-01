#![allow(warnings)]
use crate::attributes::*;
use crate::chunk::Chunk;
use crate::code::*;
use crate::list::*;
use crate::note::*;
use crate::text::*;
use crate::title::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::combinator::rest;
use nom::multi::many0;
use nom::multi::separated_list0;
use nom::IResult;
use nom::Parser;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

// NOTE: This one is not yet used, looking
// at it for a potentional of the paragraph seciton
// but haven't switched to it yet. I don't think
// it would work for both that and the title
// because of the way attributes work. but maybe
// it would becasue the attributes are only applied
// to the h1. tbd

// pub fn paragraphs(source: &str) -> IResult<&str, Option<Vec<Chunk>> {
//             let (source, attribute_list) = attributes(source)?;
//             let (return_content, content) = alt((take_until("\n-> "), rest))(source)?;
//             let (content, _) = multispace0(content)?;
//             let (remainder, mut paragraphs) =
//                 separated_list0(tag("\n\n"), take_until("\n\n"))(content)?;
//             paragraphs.push(remainder);
//             for paragraph in paragraphs.iter() {
//                 children.as_mut().unwrap().push(Chunk::P {
//                     attributes: attribute_list.clone(),
//                     children: text(paragraph).unwrap().1,
//                 });
//             }
//             Ok((return_content, block))
//         }
// }
