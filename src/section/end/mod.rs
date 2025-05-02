use crate::config::Config;
use crate::section::Section;
use crate::section::SectionParent;
use crate::section::blocks::paragraph::paragraph_block;
use crate::section::bound::SectionBound;
use crate::section::metadata::section_metadata;
use crate::span::strings::space0_line_ending_or_eof::space0_line_ending_or_eof;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::character::complete::space1;
use nom::multi::many0;
use nom::sequence::terminated;

pub fn end_section<'a>(
    source: &'a str,
    config: &'a Config,
    parent: &'a SectionParent,
    kind: &str,
) -> IResult<&'a str, Section> {
    dbg!(&source);
    let (source, _) = (tag("--"), space1, tag("/")).parse(source)?;
    let (source, _) =
        terminated(tag(kind), space0_line_ending_or_eof).parse(source)?;
    let (source, (attrs, flags)) =
        section_metadata(source, config, parent, false)?;
    let (source, _) = multispace0.parse(source)?;
    let (source, children) =
        many0(|src| paragraph_block(src, config, &SectionParent::Basic, false))
            .parse(source)?;
    Ok((
        source,
        Section::End {
            attrs,
            bound: SectionBound::Full,
            children,
            flags,
            kind: format!("{}-end", kind),
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
    fn solo_end_section_basic_test() {
        let source = r#"-- /some-end-section

bravo foxtrot tango"#;
        let config = Config::default();
        let parent = SectionParent::Page;
        let kind = "some-end-section";
        let left = Section::End {
            attrs: BTreeMap::new(),
            bound: SectionBound::Full,
            children: vec![Section::Block {
                spans: vec![Span::Text {
                    content: "bravo foxtrot tango".to_string(),
                }],
            }],
            flags: vec![],
            kind: "some-end-section-end".to_string(),
        };
        let right = end_section(source, &config, &parent, kind).unwrap().1;
        assert_eq!(left, right);
    }
}
