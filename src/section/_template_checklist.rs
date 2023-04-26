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
use nom::IResult;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum ChecklistItem {
    Basic { children: Option<Vec<String>> },
}

pub fn checklist(source: &str) -> IResult<&str, Section> {
    let (remainder, attributes) = section_attributes(source)?;
    let (remainder, _) = multispace0(remainder)?;
    let (remainder, items) = many_till(checklist_item, eof)(remainder)?;
    Ok((
        remainder,
        Section::ChecklistSection {
            attributes,
            children: Some(items.0),
        },
    ))
}

pub fn checklist_item(source: &str) -> IResult<&str, ChecklistItem> {
    let (remainder, _) = multispace0(source)?;
    let (remainder, _) = tag("[]")(remainder)?;
    let (remainder, _) = multispace1(remainder)?;
    let (remainder, captured) = alt((take_until("\n\n[]"), rest))(remainder)?;
    let (_, parts) = many_till(block, eof)(captured)?;
    let the_parts = Some(parts.0);
    let text_string = joiner(&the_parts);
    Ok((
        remainder,
        ChecklistItem::Basic {
            children: Some(text_string),
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
    pub fn basic_checklist() {
        let source = ["-> checklist", "", "[] alfa", "", "[] bravo"]
            .join("\n")
            .to_string();
        let expected = Some(
            vec![
                r#"<ul class="checklist">"#,
                r#"<li><input type="checkbox" class="checklistCheckbox" disabled /><p>alfa</p></li>"#,
                r#"<li><input type="checkbox" class="checklistCheckbox" disabled /><p>bravo</p></li>"#,
                r#"</ul>"#,
            ]
            .join("\n")
            .to_string(),
        );
        let mut u = Universe::new();
        u.env = Some(create_env("./site/templates"));
        let mut sf = SourceFile::new();
        sf.raw = Some(source);
        sf.parsed = parse(sf.raw.as_ref().unwrap().as_str()).unwrap().1;
        let output = sf.output(&u);
        assert_eq!(remove_whitespace(expected), remove_whitespace(output),);
    }
}
