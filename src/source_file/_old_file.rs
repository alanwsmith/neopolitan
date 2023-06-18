use crate::section::section::*;
use crate::source_file::joiner::joiner;
use crate::universe::universe::Universe;
use minijinja::context;
use serde::Serialize;
use std::path::PathBuf;

// THIS IS NO LONGER BEING USED, KEEPING
// FOR NOW JUST IN CASE, BUT SHOULD PROBABLY
// BE KILLED

#[derive(Debug, PartialEq, Serialize)]
pub struct SourceFile {
    pub input_path: Option<PathBuf>,
    pub parsed: Option<Vec<Section>>,
    pub raw: Option<String>,
}

impl SourceFile {
    pub fn new() -> SourceFile {
        SourceFile {
            input_path: None,
            parsed: None,
            raw: None,
        }
    }
}

impl SourceFile {
    pub fn output(&self, u: &Universe) -> Option<String> {
        let mut output_string = String::from("");
        let base = u.env.as_ref().unwrap();
        self.parsed
            .as_ref()
            .unwrap()
            .iter()
            .for_each(|section| match section {

                

// AUTO GENERATED START: Sections //

              Section::AsideSection{
                attributes,
                children,
            } => {
                let parts = joiner(children);
                output_string.push_str(
                    &base
                        .get_template("components/aside.j2")
                        .unwrap()
                        .render(context!(attributes, parts))
                        .unwrap()
                        .as_str(),
                );
            }
              Section::BlockquoteSection{
                attributes,
                children,
            } => {
                let parts = joiner(children);
                output_string.push_str(
                    &base
                        .get_template("components/blockquote.j2")
                        .unwrap()
                        .render(context!(attributes, parts))
                        .unwrap()
                        .as_str(),
                );
            }
              Section::H1Section{
                attributes,
                children,
            } => {
                let parts = joiner(children);
                output_string.push_str(
                    &base
                        .get_template("components/h1.j2")
                        .unwrap()
                        .render(context!(attributes, parts))
                        .unwrap()
                        .as_str(),
                );
            }
              Section::H2Section{
                attributes,
                children,
            } => {
                let parts = joiner(children);
                output_string.push_str(
                    &base
                        .get_template("components/h2.j2")
                        .unwrap()
                        .render(context!(attributes, parts))
                        .unwrap()
                        .as_str(),
                );
            }
              Section::H3Section{
                attributes,
                children,
            } => {
                let parts = joiner(children);
                output_string.push_str(
                    &base
                        .get_template("components/h3.j2")
                        .unwrap()
                        .render(context!(attributes, parts))
                        .unwrap()
                        .as_str(),
                );
            }
              Section::H4Section{
                attributes,
                children,
            } => {
                let parts = joiner(children);
                output_string.push_str(
                    &base
                        .get_template("components/h4.j2")
                        .unwrap()
                        .render(context!(attributes, parts))
                        .unwrap()
                        .as_str(),
                );
            }
              Section::H5Section{
                attributes,
                children,
            } => {
                let parts = joiner(children);
                output_string.push_str(
                    &base
                        .get_template("components/h5.j2")
                        .unwrap()
                        .render(context!(attributes, parts))
                        .unwrap()
                        .as_str(),
                );
            }
              Section::H6Section{
                attributes,
                children,
            } => {
                let parts = joiner(children);
                output_string.push_str(
                    &base
                        .get_template("components/h6.j2")
                        .unwrap()
                        .render(context!(attributes, parts))
                        .unwrap()
                        .as_str(),
                );
            }
              Section::NoteSection{
                attributes,
                children,
            } => {
                let parts = joiner(children);
                output_string.push_str(
                    &base
                        .get_template("components/note.j2")
                        .unwrap()
                        .render(context!(attributes, parts))
                        .unwrap()
                        .as_str(),
                );
            }
              Section::SubtitleSection{
                attributes,
                children,
            } => {
                let parts = joiner(children);
                output_string.push_str(
                    &base
                        .get_template("components/subtitle.j2")
                        .unwrap()
                        .render(context!(attributes, parts))
                        .unwrap()
                        .as_str(),
                );
            }
              Section::TitleSection{
                attributes,
                children,
            } => {
                let parts = joiner(children);
                output_string.push_str(
                    &base
                        .get_template("components/title.j2")
                        .unwrap()
                        .render(context!(attributes, parts))
                        .unwrap()
                        .as_str(),
                );
            }


// AUTO GENERATED END: Sections //




                _ => {}
            });
        Some(output_string)
    }
}

#[cfg(test)]
mod test {
    use crate::block::block::*;
    use crate::parse::parse::*;
    use crate::section::section_attributes::SectionAttribute;
    use crate::snippet::snippet::*;
    use crate::source_file::source_file::*;
    use crate::tests::remove_whitespace::remove_whitespace;
    use crate::universe::create_env::create_env;
    use crate::universe::universe::Universe;

    // This tests is a basic look at the output
    // method. Each section type has similar ones
    // specific to them

    #[test]
    pub fn basic_output_method_test() {
        let lines = ["-> title", "", "Shut the hatch"];
        let expected = Some(r#"<h1 class="title">Shut the hatch</h1>"#.to_string());
        let mut u = Universe::new();
        u.env = Some(create_env("./site/templates"));
        let mut sf = SourceFile::new();
        sf.raw = Some(lines.join("\n").to_string());
        sf.parsed = parse(sf.raw.as_ref().unwrap().as_str()).unwrap().1;
        let output = sf.output(&u);
        assert_eq!(remove_whitespace(expected), remove_whitespace(output),);
    }

    // The tests below are for looking at the data
    // structure

    #[test]
    pub fn basic_title_test() {
        let mut sf = SourceFile::new();
        let lines = ["-> title", "", "Dip the pail once"];
        let expected = Some(vec![Section::TitleSection {
            attributes: None,
            children: Some(vec![Block::Text {
                snippets: Some(vec![Snippet::Plain {
                    text: Some("Dip the pail once".to_string()),
                }]),
            }]),
        }]);
        sf.raw = Some(lines.join("\n").to_string());
        sf.parsed = parse(sf.raw.unwrap().as_str()).unwrap().1;
        assert_eq!(expected, sf.parsed);
    }

    #[test]
    pub fn basic_title_plus_lines_test() {
        let mut sf = SourceFile::new();
        let lines = ["-> title", "", "Dip the pail once", "", "Draw the chart"];
        let expected = Some(vec![Section::TitleSection {
            attributes: None,
            children: Some(vec![
                Block::Text {
                    snippets: Some(vec![Snippet::Plain {
                        text: Some("Dip the pail once".to_string()),
                    }]),
                },
                Block::Text {
                    snippets: Some(vec![Snippet::Plain {
                        text: Some("Draw the chart".to_string()),
                    }]),
                },
            ]),
        }]);
        sf.raw = Some(lines.join("\n").to_string());
        sf.parsed = parse(sf.raw.unwrap().as_str()).unwrap().1;
        assert_eq!(expected, sf.parsed);
    }

    #[test]
    pub fn attributes() {
        let mut sf = SourceFile::new();
        let lines = ["-> title", ">> id: bravo", "", "Set The Piece"];
        let expected = Some(vec![Section::TitleSection {
            attributes: Some(vec![SectionAttribute::Attribute {
                key: Some("id".to_string()),
                value: Some("bravo".to_string()),
            }]),
            children: Some(vec![Block::Text {
                snippets: Some(vec![Snippet::Plain {
                    text: Some("Set The Piece".to_string()),
                }]),
            }]),
        }]);
        sf.raw = Some(lines.join("\n").to_string());
        sf.parsed = parse_dev(sf.raw.unwrap().as_str()).unwrap().1;
        assert_eq!(expected, sf.parsed);
    }

    // #[test]
    // pub fn test_file_type() {
    //     let mut sf = SourceFile::new();
    //     let lines = ["-> title", "", "Set The Piece", "", "-> attributes", ">> type: testing"];
    //     assert_eq!(1, 2);
    // }

}
