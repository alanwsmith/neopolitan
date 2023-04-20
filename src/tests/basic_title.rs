#![allow(warnings)]
use crate::block::block::*;
use crate::parse::parse::parse;
use crate::section::section::*;
use crate::source_file::source_file::SourceFile;
use crate::universe::create_env::*;
use crate::universe::universe::Universe;
use minijinja::context;
// use minijinja::Environment;

#[test]
pub fn basic_title() {
    let mut u = Universe::new();
    u.env = Some(create_env("./src/tests/templates"));
    let lines = ["-> title", "", "Pick The Rose"];
    let expected = Some(vec![r#"<h1 class="title">Pick The Rose</h1>"#.to_string()]);
    let source = lines.join("\n");
    let mut sf = SourceFile::new();
    sf.raw_data = Some(source.to_string());
    sf.parsed = parse(sf.raw_data.unwrap().as_str()).unwrap().1;
    sf.output_chunks = Some(vec![]);
    sf.parsed.unwrap().iter().for_each(|section| {
        match section {
            Section::TitleSection {
                attributes,
                children,
            } => {
                let structure = u.env.as_ref().unwrap().get_template("title.j2").unwrap();
                sf.output_chunks.as_mut().unwrap().push(
                    structure
                        .render(context!(attributes, children))
                        .unwrap()
                        .to_string(),
                );
            }
            _ => {}
        }
        ()
    });
    let expected_string: String = expected
        .unwrap()
        .join("\n")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let output_string: String = sf
        .output_chunks
        .unwrap()
        .join("\n")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    assert_eq!(expected_string, output_string);
}
