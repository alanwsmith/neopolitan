use crate::block::block::*;
use crate::section::note::*;
use crate::section::section_attributes::SectionAttribute;
use crate::section::subtitle::*;
use crate::section::title::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Section {
    NoteSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    SubtitleSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    TitleSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    Placeholder,
}

pub fn section(source: &str) -> IResult<&str, Section> {
    let (remainder, _) = multispace0(source)?;
    let (remainder, _) = tag("-> ")(remainder)?;
    let (remainder, section) = alt((
        tuple((
            tag("note"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| note(t.3).unwrap().1),
        tuple((
            tag("title"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| title(t.3).unwrap().1),
        tuple((
            tag("subtitle"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| subtitle(t.3).unwrap().1),
    ))(remainder)?;
    Ok((remainder, section))
}



//#[test] 
//pub fn aside_basic() {
//    // let mut u = Universe::new();
//    // u.env = Some(create_env("./src/tests/templates"));
//    let lines = ["-> aside", "", "Take the match"];
//    let expected = Some(vec![r#"<aside><p>Take the match</p></aside>"#.to_string()]);
//    let source = lines.join("\n");
//    // let mut sf = SourceFile::new();
//    //sf.raw_data = Some(source.to_string());
//    // sf.parsed = parse(sf.raw_data.unwrap().as_str()).unwrap().1;
//    sf.output_chunks = Some(vec![]);
//    sf.parsed.unwrap().iter().for_each(|section| {
//        match section {
//            Section::TitleSection {
//                attributes,
//                children,
//            } => {
//                let structure = u.env.as_ref().unwrap().get_template("title.j2").unwrap();
//                sf.output_chunks.as_mut().unwrap().push(
//                    structure
//                        .render(context!(attributes, children))
//                        .unwrap()
//                        .to_string(),
//                );
//            }
//            _ => {}
//        }
//        ()
//    });
//    let expected_string: String = expected
//        .unwrap()
//        .join("\n")
//        .chars()
//        .filter(|c| !c.is_whitespace())
//        .collect();
//    let output_string: String = sf
//        .output_chunks
//        .unwrap()
//        .join("\n")
//        .chars()
//        .filter(|c| !c.is_whitespace())
//        .collect();
//    assert_eq!(expected_string, output_string);
//}  
//}
