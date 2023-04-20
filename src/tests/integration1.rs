#![allow(unused_imports)]
use crate::block::block::*;
use crate::parse::parse::parse;
use crate::section::section::*;
use crate::source_file::source_file::SourceFile;
use crate::universe::create_env::*;
use crate::universe::universe::Universe;
use minijinja::context;
use std::fs;
// use minijinja::Environment;

#[test]
pub fn integration1() {
    let mut u = Universe::new();
    u.env = Some(create_env("./src/tests/templates"));

    let source = fs::read_to_string("./src/tests/integration1.neo").unwrap();
    // dbg!(&source);
    let mut sf = SourceFile::new();
    sf.raw_data = Some(source);
    sf.parsed = parse(sf.raw_data.unwrap().as_str()).unwrap().1;
    // dbg!(&sf.parsed);
    sf.output_chunks = Some(vec![]);

    // THis should move to a function
    sf.parsed.unwrap().iter().for_each(|section| {
        match section {
            Section::NoteSection {
                attributes,
                children,
            } => {
                let structure = u.env.as_ref().unwrap().get_template("note.j2").unwrap();
                sf.output_chunks.as_mut().unwrap().push(
                    structure
                        .render(context!(attributes, children))
                        .unwrap()
                        .to_string(),
                );
            }
            Section::SubtitleSection {
                attributes,
                children,
            } => {
                let structure = u.env.as_ref().unwrap().get_template("subtitle.j2").unwrap();
                sf.output_chunks.as_mut().unwrap().push(
                    structure
                        .render(context!(attributes, children))
                        .unwrap()
                        .to_string(),
                );
            }

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

    // dbg!(sf.output_chunks);

    let content = sf.output_chunks.unwrap().join("\n");
    let tmpl = u
        .env
        .as_ref()
        .unwrap()
        .get_template("wrappers/default.j2")
        .unwrap();
    let output = tmpl.render(context!(content)).unwrap().to_string();
    fs::write("sites/test/index.html", output).unwrap();
    // dbg!(output);

    // let lines = ["-> title", "", "Pick The Rose"];
    // let expected = Some(vec![r#"<h1 class="title">Pick The Rose</h1>"#.to_string()]);
    // let source = lines.join("\n");
    // let mut sf = SourceFile::new();
    // sf.raw_data = Some(source.to_string());
    // sf.parsed = parse(sf.raw_data.unwrap().as_str()).unwrap().1;
    // sf.output_chunks = Some(vec![]);
    // sf.parsed.unwrap().iter().for_each(|section| {
    //     match section {
    //         Section::TitleSection {
    //             attributes,
    //             children,
    //         } => {
    //             let structure = u.env.as_ref().unwrap().get_template("title.j2").unwrap();
    //             sf.output_chunks.as_mut().unwrap().push(
    //                 structure
    //                     .render(context!(attributes, children))
    //                     .unwrap()
    //                     .to_string(),
    //             );
    //         }
    //         _ => {}
    //     }
    //     ()
    // });
    // let expected_string: String = expected
    //     .unwrap()
    //     .join("\n")
    //     .chars()
    //     .filter(|c| !c.is_whitespace())
    //     .collect();
    // let output_string: String = sf
    //     .output_chunks
    //     .unwrap()
    //     .join("\n")
    //     .chars()
    //     .filter(|c| !c.is_whitespace())
    //     .collect();
    // assert_eq!(expected_string, output_string);
}
