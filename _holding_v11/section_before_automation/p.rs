use crate::block::block::*;
use crate::section::section::*;
use crate::section::section_attributes::*;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub fn p(source: &str) -> IResult<&str, Section> {
    let (remainder, attributes) = section_attributes(source)?;
    let (remainder, _) = multispace0(remainder)?;
    let (remainder, blocks) = many_till(block, eof)(remainder)?;
    Ok((
        remainder,
        Section::ParagraphsSection {
            attributes,
            children: Some(blocks.0),
        },
    ))
}

// #[cfg(test)]
// mod test {
//     use crate::parse::parse::*;
//     use crate::source_file::source_file::*;
//     use crate::tests::remove_whitespace::remove_whitespace;
//     use crate::universe::create_env::create_env;
//     use crate::universe::universe::Universe;
//     #[test]
//     pub fn core_test_p() {
//         let source = [
//             "-> p",
//             ">> class: bravo",
//             "",
//             "Hold the hammer",
//             "",
//             "Heave the line",
//         ]
//         .join("\n")
//         .to_string();
//         let expected = Some(
//             vec![
//                 r#"<p class="bravo">Hold the hammer</p>"#,
//                 r#"<p class="bravo">Heave the line</p>"#,
//             ]
//             .join("\n")
//             .to_string(),
//         );
//         let mut u = Universe::new();
//         u.env = Some(create_env("./site/templates"));
//         let mut sf = SourceFile::new();
//         sf.raw = Some(source);
//         sf.parsed = parse(sf.raw.as_ref().unwrap().as_str()).unwrap().1;
//         let output = sf.output(&u);
//         assert_eq!(remove_whitespace(expected), remove_whitespace(output),);
//     }
// }
