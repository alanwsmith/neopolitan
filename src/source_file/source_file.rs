use crate::section::section::*;
use crate::source_file::attributes_basic::*;
use crate::source_file::attributes_with_class::*;
use crate::source_file::joiner::joiner;
use crate::universe::universe::Universe;
use minijinja::context;
use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::combinator::rest;
use nom::IResult;
use serde::Serialize;
use std::path::PathBuf;
use nom::bytes::complete::tag;
use nom::bytes::complete::is_not;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SourceFile {
    pub output_path: Option<PathBuf>,
    pub parsed: Option<Vec<Section>>,
    pub raw: Option<String>,
    pub raw_path: Option<PathBuf>,
    pub file_type: Option<String>,
}

impl SourceFile {
    pub fn file_type(&self) -> IResult<&str, Option<String>> {
        let tmp_string = self.raw.as_ref().unwrap().as_str();
        let (a, _b) = alt((take_until(">> type: "), rest))(tmp_string)?;
        if a == "" {
            Ok(("", Some(String::from("default"))))
        }
        else {
            let (a, _b) = tag(">> type: ")(a)?;
            let (_a, b) = is_not(" \t\n")(a)?;
            Ok(("", Some(String::from(b))))
        }
    }
}

impl SourceFile {
    pub fn new() -> SourceFile {
        SourceFile {
            output_path: None,
            parsed: None,
            raw: None,
            raw_path: None,
            file_type: Some(String::from("REMOVETHIS")),
        }
    }
}

impl SourceFile {
    pub fn output(&self, u: &Universe) -> Option<String> {
        let mut output_string = String::from("");
        let base = u.env.as_ref().unwrap();
        if let Some(parsed) = self.parsed.as_ref() {
            parsed.iter().for_each(|section| match section {
                Section::NotesSection {
                    attributes,
                    children,
                } => {
                    let attributes_string = attributes_with_class(attributes, "notes");
                    output_string.push_str(
                        &base
                            .get_template("components/notes.j2")
                            .unwrap()
                            .render(context!(attributes_string, children))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::OrderedListSection {
                    attributes,
                    children,
                } => {
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/olist.j2")
                            .unwrap()
                            .render(context!(attributes_string, children))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::ListSection {
                    attributes,
                    children,
                } => {
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/list.j2")
                            .unwrap()
                            .render(context!(attributes_string, children))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::ChecklistSection {
                    attributes,
                    children,
                } => {
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/checklist.j2")
                            .unwrap()
                            .render(context!(attributes_string, children))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::AsideSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/aside.j2")
                            .unwrap()
                            .render(context!(attributes_string, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::BlockquoteSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/blockquote.j2")
                            .unwrap()
                            .render(context!(attributes_string, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::CanvasSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/canvas.j2")
                            .unwrap()
                            .render(context!(attributes_string, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::CodeSection {
                    attributes,
                    attributes_string,
                    language,
                    raw,
                    title,
                } => {
                    output_string.push_str(
                        &base
                            .get_template("components/code.j2")
                            .unwrap()
                            .render(context!(
                                attributes,
                                attributes_string,
                                language,
                                raw,
                                title
                            ))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::DetailsSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/details.j2")
                            .unwrap()
                            .render(context!(attributes_string, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::StartDivSection { attributes, html } => {
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/startdiv.j2")
                            .unwrap()
                            .render(context!(attributes_string, html))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::DescriptionListSection {
                    attributes,
                    children,
                } => {
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/dlist.j2")
                            .unwrap()
                            .render(context!(attributes_string, children))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::H1Section {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/h1.j2")
                            .unwrap()
                            .render(context!(attributes_string, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::H2Section {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/h2.j2")
                            .unwrap()
                            .render(context!(attributes_string, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::H3Section {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/h3.j2")
                            .unwrap()
                            .render(context!(attributes_string, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::H4Section {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/h4.j2")
                            .unwrap()
                            .render(context!(attributes_string, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::H5Section {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/h5.j2")
                            .unwrap()
                            .render(context!(attributes_string, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::H6Section {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/h6.j2")
                            .unwrap()
                            .render(context!(attributes_string, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::HRSection { attributes } => {
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/hr.j2")
                            .unwrap()
                            .render(context!(attributes_string))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::ImageSection {
                    alt_text,
                    attributes,
                    src,
                } => {
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/image.j2")
                            .unwrap()
                            .render(context!(attributes_string, alt_text, src))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::MenuSection {
                    attributes,
                    children,
                } => {
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/menu.j2")
                            .unwrap()
                            .render(context!(attributes_string, children))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::NavSection {
                    attributes,
                    children,
                } => {
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/nav.j2")
                            .unwrap()
                            .render(context!(attributes_string, children))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::NoteSection {
                    attributes,
                    children,
                    title,
                } => {
                    let parts = joiner(children);
                    let attributes_string = attributes_with_class(attributes, "note");
                    output_string.push_str(
                        &base
                            .get_template("components/note.j2")
                            .unwrap()
                            .render(context!(attributes_string, parts, title))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::ObjectSection { attributes } => {
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/object.j2")
                            .unwrap()
                            .render(context!(attributes_string))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::ParagraphsSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/p.j2")
                            .unwrap()
                            .render(context!(attributes_string, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::PreSection { attributes, raw } => {
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/pre.j2")
                            .unwrap()
                            .render(context!(attributes_string, raw))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::ResultsSection { attributes, raw } => {
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/results.j2")
                            .unwrap()
                            .render(context!(attributes_string, raw))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::CodeStartEndSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/startcode.j2")
                            .unwrap()
                            .render(context!(attributes_string, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::SubtitleSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    let attributes_string = attributes_with_class(attributes, "subtitle");
                    output_string.push_str(
                        &base
                            .get_template("components/subtitle.j2")
                            .unwrap()
                            .render(context!(attributes_string, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::TableSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/table.j2")
                            .unwrap()
                            .render(context!(attributes_string, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::TextareaSection { attributes, raw } => {
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/textarea.j2")
                            .unwrap()
                            .render(context!(attributes_string, raw))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::TitleSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    let attributes_string = attributes_with_class(attributes, "title");
                    output_string.push_str(
                        &base
                            .get_template("components/title.j2")
                            .unwrap()
                            .render(context!(attributes_string, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::TodoSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    let attributes_string = attributes_with_class(attributes, "todo");
                    output_string.push_str(
                        &base
                            .get_template("components/todo.j2")
                            .unwrap()
                            .render(context!(attributes_string, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::VimeoSection { attributes, id } => {
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/vimeo.j2")
                            .unwrap()
                            .render(context!(attributes_string, id))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::WarningSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    let attributes_string = attributes_with_class(attributes, "warning");
                    output_string.push_str(
                        &base
                            .get_template("components/warning.j2")
                            .unwrap()
                            .render(context!(attributes_string, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::YouTubeSection { attributes, id } => {
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/youtube.j2")
                            .unwrap()
                            .render(context!(attributes_string, id))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::AudioSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/audio.j2")
                            .unwrap()
                            .render(context!(attributes_string, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::TodosSection {
                    attributes,
                    children,
                } => {
                    let attributes_string = attributes_with_class(attributes, "todos");
                    output_string.push_str(
                        &base
                            .get_template("components/todos.j2")
                            .unwrap()
                            .render(context!(attributes_string, children))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::NeoExampleStartEndSection {
                    attributes,
                    html,
                    raw,
                } => {
                    let attributes_string = attributes_basic(attributes);
                    output_string.push_str(
                        &base
                            .get_template("components/startneoexample.j2")
                            .unwrap()
                            .render(context!(attributes_string, html, raw))
                            .unwrap()
                            .as_str(),
                    );
                }
                _ => {}
            });
        }
        Some(output_string)
    }
}

#[cfg(test)]
mod test {
    use crate::block::block::*;
    use crate::parse::parse::*;
    use crate::section::section_attributes::SectionAttribute;
    use crate::snippet::snippet_enum::*;
    use crate::source_file::source_file::*;
    // use crate::tests::remove_whitespace::remove_whitespace;
    // use crate::universe::create_env::create_env;
    // use crate::universe::universe::Universe;

    // This tests is a basic look at the output
    // method. Each section type has similar ones
    // specific to them

    // #[test]
    // pub fn basic_output_method_test() {
    //     let lines = ["-> title", "", "Shut the hatch"];
    //     let expected = Some(r#"<h1 class="title">Shut the hatch</h1>"#.to_string());
    //     let mut u = Universe::new();
    //     u.env = Some(create_env("./site/templates"));
    //     let mut sf = SourceFile::new();
    //     sf.raw = Some(lines.join("\n").to_string());
    //     sf.parsed = parse(sf.raw.as_ref().unwrap().as_str()).unwrap().1;
    //     let output = sf.output(&u);
    //     assert_eq!(remove_whitespace(expected), remove_whitespace(output),);
    // }

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
        sf.parsed = parse(sf.raw.unwrap().as_str()).unwrap().1;
        assert_eq!(expected, sf.parsed);
    }

    #[test]
    pub fn test_file_type() {
        let mut sf = SourceFile::new();
        let lines = [
            "-> title",
            "",
            "Quick brown fox",
            "",
            "-> attributes",
            ">> type: testing",
        ];
        sf.raw = Some(lines.join("\n").to_string());
        let expected = Some(String::from("testing"));
        let result = sf.file_type().unwrap().1;
        assert_eq!(expected, result)
    }

    #[test]
    pub fn test_default_file_type() {
        let mut sf = SourceFile::new();
        let lines = [
            "-> title",
            "",
            "Quick brown fox",
        ];
        sf.raw = Some(lines.join("\n").to_string());
        let expected = Some(String::from("default"));
        let result = sf.file_type().unwrap().1;
        assert_eq!(expected, result)
    }

    #[test]
    pub fn test_file_type_with_following_lines() {
        let mut sf = SourceFile::new();
        let lines = [
            "-> title",
            "",
            "Quick brown fox",
            "",
            "-> attributes",
            ">> type: second_test",
            ">> status: unpublished",
        ];
        sf.raw = Some(lines.join("\n").to_string());
        let expected = Some(String::from("second_test"));
        let result = sf.file_type().unwrap().1;
        assert_eq!(expected, result)
    }

}
