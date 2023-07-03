use crate::block::block::*;
use crate::section::section::*;
use crate::section::section_attributes::*;
use crate::source_file::joiner::joiner;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum DescriptionListItem {
    Basic {
        dt_content: Option<Vec<String>>,
        dd_content: Option<Vec<String>>,
    },
}

pub fn dlist(source: &str) -> IResult<&str, Section> {
    let (remainder, attributes) = section_attributes(source)?;
    let (remainder, _) = multispace0(remainder)?;
    let (remainder, items) = many_till(dlist_item, eof)(remainder)?;
    Ok((
        remainder,
        Section::DescriptionListSection {
            attributes,
            children: Some(items.0),
        },
    ))
}

pub fn dlist_item(source: &str) -> IResult<&str, DescriptionListItem> {
    let (remainder, _) = multispace0(source)?;
    let (remainder, _) = tag("-")(remainder)?;
    let (remainder, _) = multispace1(remainder)?;
    let (remainder, dt_capture) = alt((
        tuple((take_until("\n\n>"), tag("\n\n>"))).map(|x| x.0),
        rest,
    ))(remainder)?;
    let (_, dt_parts_tuple) = many_till(block, eof)(dt_capture)?;
    let dt_parts = Some(dt_parts_tuple.0);
    let dt_string = joiner(&dt_parts);
    let (remainder, _) = multispace1(remainder)?;
    let (remainder, dd_capture) = alt((take_until("\n\n-"), rest))(remainder)?;
    let (_, dd_parts_tuple) = many_till(block, eof)(dd_capture)?;
    let dd_parts = Some(dd_parts_tuple.0);
    let dd_string = joiner(&dd_parts);
    Ok((
        remainder,
        DescriptionListItem::Basic {
            dt_content: Some(dt_string),
            dd_content: Some(dd_string),
        },
    ))
}

#[cfg(test)]
mod test {
    use crate::parse::parse::*;
    use crate::source_file::source_file::*;
    use crate::tests::remove_whitespace::remove_whitespace;
    use crate::universe::create_env::create_env;
    use crate::universe::universe::Universe;

    #[test]
    pub fn basic_dlist() {
        let source = ["-> dlist", "", "- alfa", "", "> bravo"]
            .join("\n")
            .to_string();
        let expected = Some(
            vec![
                r#"<dl>"#,
                r#"<dt><p>alfa</p></dt>"#,
                r#"<dd><p>bravo</p></dd>"#,
                r#"</dl>"#,
            ]
            .join("\n")
            .to_string(),
        );
        let mut u = Universe::new();
        u.env = Some(create_env("./templates"));
        let mut sf = SourceFile::new();
        sf.raw = Some(source);
        sf.parsed = parse(sf.raw.as_ref().unwrap().as_str()).unwrap().1;
        let output = sf.output(&u);
        assert_eq!(remove_whitespace(expected), remove_whitespace(output),);
    }
}
