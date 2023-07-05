use crate::block::Block;
use crate::section::Section;
use crate::snippet::Snippet;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::sequence::tuple;
use nom::IResult;

pub fn title(source: &str) -> IResult<&str, Section> {
    let (source, _) = tuple((
        tag("-> title"),
        not_line_ending,
        line_ending,
    ))(source.trim())?;
    let (source, _) = line_ending(source)?;
    let (source, captured) = not_line_ending(source)?;

    let result = Section::Title {
        attrs: vec![],
        headline: Block::Headline {
            content: vec![Snippet::Text {
                string: captured.to_string(),
            }],
        },
        paragraphs: vec![],
    };
    Ok((source, result))
}

