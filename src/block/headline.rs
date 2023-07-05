
use crate::block::Block;
use nom::IResult;
use crate::snippet::Snippet;


pub fn headline(source: &str) -> IResult<&str, Block> {
Ok((source, Block::Headline{content: vec![Snippet::Text {string: "hello world".to_string()}]}))
}