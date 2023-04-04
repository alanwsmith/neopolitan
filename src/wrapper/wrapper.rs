#![allow(unused_imports)]
use crate::block::block::Block;
use crate::section::section::*;
use crate::section::section_attributes::SectionAttribute;
// use crate::wrapper::wrapper::content;
use crate::content::content::*;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::opt;
use nom::combinator::rest;
use nom::error::Error;
use nom::multi::many0;
use nom::multi::many_till;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq)]
pub enum Wrapper {
    Page { children: Option<Vec<Section>> },
}
