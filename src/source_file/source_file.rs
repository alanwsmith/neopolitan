use crate::section::section::*;
use crate::source_file::attributes_basic::*;
use crate::source_file::joiner::joiner;
use crate::universe::universe::Universe;
use minijinja::context;
use serde::Serialize;
use std::path::PathBuf;

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
                    output_string.push_str(
                        &base
                            .get_template("components/blockquote.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::CanvasSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/canvas.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::ChecklistSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/checklist.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::CodeSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/code.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::DetailsSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/details.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::DivSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/div.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::DescriptionListSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/dlist.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::FigureSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/figure.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::H1Section {
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
                Section::H2Section {
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
                Section::H3Section {
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
                Section::H4Section {
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
                Section::H5Section {
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
                Section::H6Section {
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
                Section::HRSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/hr.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::ImageSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/image.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::ListSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/list.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::MenuSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/menu.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::NavSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/nav.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::NoteSection {
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
                Section::NotesSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/notes.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::ObjectSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/object.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::OrderedListSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/olist.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::ParagraphsSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/p.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::PictureSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/picture.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::PreSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/pre.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::ResultsSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/results.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::CodeStartEndSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/startcode.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::SubtitleSection {
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
                Section::TableSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/table.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::TextareaSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/textarea.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::TitleSection {
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
                Section::TodoSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/todo.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::VimeoSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/vimeo.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::WarningSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/warning.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::YouTubeSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/youtube.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::VideoSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/video.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }

                Section::AttributesSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/attributes.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::BlurbSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/blurb.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::CategoriesSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/categories.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::CommentSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/comment.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::CSSSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/css.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::ExternalSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/ext.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::FootnoteSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/footnote.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::HeadSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/head.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::HTMLSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/html.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::IncludeSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/include.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::ReferenceSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/reference.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::ScriptSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/script.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }
                Section::WidgetSection {
                    attributes,
                    children,
                } => {
                    let parts = joiner(children);
                    output_string.push_str(
                        &base
                            .get_template("components/widget.j2")
                            .unwrap()
                            .render(context!(attributes, parts))
                            .unwrap()
                            .as_str(),
                    );
                }

                // AUTO GENERATED START: Sections //
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
}
