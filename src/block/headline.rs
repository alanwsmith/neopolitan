use crate::block::Block;
use crate::snippet::Snippet;
use nom::IResult;

pub fn headline(source: &str) -> IResult<&str, Block> {
    Ok((
        source,
        Block::Headline {
            content: vec![Snippet::Text {
                string: source.to_string(),
            }],
        },
    ))
}

