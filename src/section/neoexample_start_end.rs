use crate::block::block::*;
use crate::parse::parse::parse;
use crate::section::section::*;
use crate::section::section_attributes::*;
use crate::source_file::source_file::SourceFile;
use crate::universe::create_env::create_env;
use crate::universe::universe::Universe;
use html_escape;
use minijinja::context;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub fn neoexample_start_end(source: &str) -> IResult<&str, Section> {
    let (remainder, attributes) = section_attributes(source)?;
    let (raw, _) = multispace0(remainder)?;
    let (remainder, _blocks) = many_till(block, eof)(raw)?;

    let escaped_raw = html_escape::encode_text(raw);

    let mut u = Universe::new();
    u.env = Some(create_env("./site/templates"));
    u.env
        .as_mut()
        .unwrap()
        .add_template("neoexample_start_end_hard_coded", "{{content}}")
        .unwrap();

    let mut sf = SourceFile::new();
    sf.raw = Some(raw.to_string());
    sf.parsed = parse(sf.raw.as_ref().unwrap().as_str()).unwrap().1;

    let wrapper = u
        .env
        .as_ref()
        .unwrap()
        .get_template("neoexample_start_end_hard_coded")
        .unwrap();
    let out = wrapper
        .render(context!(content => sf.output(&u).unwrap()))
        .unwrap()
        .to_string();

    let escaped_html = html_escape::encode_text(&out);

    Ok((
        remainder,
        Section::NeoExampleStartEndSection {
            attributes,
            html: Some(escaped_html.to_string()),
            raw: Some(escaped_raw.to_string()),
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
//     pub fn aside_with_class_and_another_attribute() {
//         let source = [
//             "-> aside",
//             ">> class: delta",
//             ">> id: bravo",
//             "",
//             "Hold the hammer",
//             "",
//             "Heave the line",
//         ]
//         .join("\n")
//         .to_string();
//         let expected = Some(
//             vec![
//                 r#"<aside class="delta" id="bravo">"#,
//                 r#"<p>Hold the hammer</p>"#,
//                 r#"<p>Heave the line</p>"#,
//                 r#"</aside>"#,
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
