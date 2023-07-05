use crate::block::headline::headline;
use crate::sec_attr::sec_attrs;
use crate::section::Section;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
use nom::sequence::tuple;
use nom::IResult;

pub fn title(source: &str) -> IResult<&str, Section> {
    let (source, _) = tuple((
        tag("-> title"),
        not_line_ending,
        line_ending,
    ))(source.trim())?;
    let (source, content) =
        alt((take_until("\n\n->"), rest))(source.trim())?;
    let (content, attrs) = sec_attrs(content.trim())?;
    let (_, headline) = headline(content.trim())?;
    let result = Section::Title {
        attrs,
        headline,
        paragraphs: vec![],
    };
    Ok((source, result))
}


#[cfg(test)]

mod test {
    use crate::section::Section;
    use crate::block::Block;
    use crate::snippet::Snippet;
    use super::*;

    #[test]
    pub fn basic_title() {
        let lines = vec!["-> title", "", "alfa bravo"].join("\n");
        let expected = Section::Title {
            attrs: vec![],
            headline: Block::Headline{ content: vec![Snippet::Text { string: "alfa bravo".to_string() } ]},
            paragraphs: vec![]
        };
        assert_eq!(expected, title(lines.as_str()).unwrap().1);
    }
}
