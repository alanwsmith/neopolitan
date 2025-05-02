use crate::config::Config;
use crate::section::Section;
use crate::section::blocks::paragraph::paragraph_block;
use crate::section::bound::SectionBound;
use crate::section::metadata::section_metadata;
use crate::section::parent::SectionParent;
use crate::span::strings::space0_line_ending_or_eof::space0_line_ending_or_eof;
use nom::Parser;
use nom::bytes::complete::is_not;
use nom::character::complete::multispace0;
use nom::character::complete::space1;
use nom::multi::many0;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::{IResult, bytes::complete::tag};

pub fn basic_section_full<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a SectionParent,
    debug: bool,
) -> IResult<&'a str, Section> {
    let (source, _) = pair(tag("--"), space1).parse(source)?;
    let (source, kind) =
        terminated(is_not("/ \t\r\n"), space0_line_ending_or_eof)
            .parse(source)?;
    let (source, (attrs, flags)) =
        section_metadata(source, config, parent, debug)?;
    let (source, _) = multispace0.parse(source)?;
    let (source, children) =
        many0(|src| paragraph_block(src, config, &SectionParent::Basic, debug))
            .parse(source)?;
    Ok((
        source,
        Section::Basic {
            attrs,
            bound: SectionBound::Full,
            children,
            end_section: None,
            flags,
            kind: kind.to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    #[test]
    fn basic_test() {
        let source = r#"-- title

bravo foxtrot tango"#;
        let config = Config::default();
        let parent = SectionParent::Page;
        let debug = false;
        let left = Section::Basic {
            attrs: BTreeMap::new(),
            bound: SectionBound::Full,
            children: vec![Section::Block {
                spans: vec![Span::Text {
                    content: "bravo foxtrot tango".to_string(),
                }],
            }],
            end_section: None,
            flags: vec![],
            kind: "title".to_string(),
        };
        let right = basic_section_full(source, &config, &parent, debug)
            .unwrap()
            .1;
        assert_eq!(left, right);
    }
}
