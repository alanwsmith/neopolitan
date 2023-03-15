#![allow(unused_imports)]
use crate::page::Page;
use crate::parse::parse;
use crate::parse_dev::parse_dev;

pub fn parse_switch(source: &str) -> Page {
    // return parse_dev(source);
    return parse(source);
}
