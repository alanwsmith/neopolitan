#![allow(dead_code)]
#![allow(unused_variables)]
use crate::block::block::Block;
use crate::block::block::Snippet;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq)]
pub struct SourceFile {
    raw_data: Option<String>,
    sections: Option<Vec<Section>>,
}

impl SourceFile {
    pub fn new() -> SourceFile {
        SourceFile {
            raw_data: None,
            sections: None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Section {
    TitleSection {
        attributes: Option<Vec<(Option<String>, Option<String>)>>,
        children: Option<Vec<Block>>,
    },
    Placeholder,
}

#[cfg(test)]
mod test {
    use crate::source_file::source_file::*;

    #[test]
    pub fn basic_title_test() {
        let mut sf = SourceFile::new();
        let lines = ["-> title", "", "Dip the pail once"];
        let expected = Some(vec![Section::TitleSection {
            attributes: None,
            children: Some(vec![Block::Text {
                snippets: Some(vec![Snippet::Plain {
                    text: Some("Dip the pail once".to_string()),
                }]),
            }]),
        }]);
        sf.raw_data = Some(lines.join("\n").to_string());
        sf.sections = parse(sf.raw_data.unwrap().as_str()).unwrap().1;
        assert_eq!(expected, sf.sections);
    }

    #[test]
    pub fn basic_title_plus_lines_test() {
        let mut sf = SourceFile::new();
        let lines = ["-> title", "", "Dip the pail once", "", "Draw the chart"];
        let expected = Some(vec![Section::TitleSection {
            attributes: None,
            children: Some(vec![Block::Text {
                snippets: Some(vec![Snippet::Plain {
                    text: Some("Dip the pail once".to_string()),
                }]),
            }]),
        }]);
        sf.raw_data = Some(lines.join("\n").to_string());
        sf.sections = parse(sf.raw_data.unwrap().as_str()).unwrap().1;
        assert_eq!(expected, sf.sections);
    }
}

pub fn parse(source: &str) -> IResult<&str, Option<Vec<Section>>> {
    let (_, sections) = many_till(section, eof)(source)?;
    Ok(("", Some(sections.0)))
}

pub fn section(source: &str) -> IResult<&str, Section> {
    let (remainder, _) = multispace0(source)?;
    let (remainder, _) = tag("-> ")(remainder)?;
    let (remainder, section) = alt((tuple((
        tag("title"),
        not_line_ending,
        line_ending,
        alt((take_until("\n\n-> "), rest)),
    ))
    .map(|t| title(t.3).unwrap().1),))(remainder)?;
    Ok(("", section))
}

pub fn title(source: &str) -> IResult<&str, Section> {
    let (remainder, _) = multispace0(source)?;
    let (_, blocks) = many_till(block, eof)(remainder)?;
    Ok((
        "",
        Section::TitleSection {
            attributes: None,
            children: Some(blocks.0),
        },
    ))
}

pub fn block(source: &str) -> IResult<&str, Block> {
    let (remainder, captured) = alt((take_until("\n\n"), rest))(source)?;
    dbg!(captured);
    let (remainder, snippets) = many_till(snippet, eof)(captured)?;
    let return_block = Block::Text {
        snippets: Some(snippets.0),
    };
    Ok((remainder.trim(), return_block))
}

pub fn snippet(source: &str) -> IResult<&str, Snippet> {
    dbg!(source);
    Ok((
        "",
        Snippet::Plain {
            text: Some(source.to_string()),
        },
    ))
}
