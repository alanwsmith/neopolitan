use crate::section::lib::get_title_from_attributes::*;
use crate::section::section::*;
use crate::section::section_attributes::*;
use crate::source_file::attributes_basic::attributes_basic;
use html_escape;
use nom::character::complete::multispace0;
use nom::IResult;
// use syntect::html;
// use syntect::easy::HighlightLines;
// use syntect::highlighting::ThemeSet;
// use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};
// use syntect::parsing::SyntaxSet;

pub fn code(source: &str) -> IResult<&str, Section> {
    let (remainder, initial_attributes) = section_attributes(source)?;
    let (remainder, _) = multispace0(remainder)?;
    let title = get_title_from_attributes(&initial_attributes);
    let mut language: Option<String> = None;
    // This pulls off the first attribute if there is no
    // value and uses it as the language for code highlighting
    let mut tmp_attrs: Vec<SectionAttribute> = vec![];
    let raw = Some(html_escape::encode_text(remainder).to_string());
    match initial_attributes {
        Some(attrs) => {
            attrs.iter().enumerate().for_each(|(a, b)| {
                if a == 0 {
                    match b {
                        SectionAttribute::Attribute { key, value } => match value {
                            Some(_) => {
                                tmp_attrs.push(SectionAttribute::Attribute {
                                    key: Some(key.as_ref().unwrap().to_string()),
                                    value: Some(value.as_ref().unwrap().to_string()),
                                });
                                // raw = Some(html_escape::encode_text(remainder).to_string());
                            }
                            None => {
                                // let ps = SyntaxSet::load_defaults_newlines();
                                // let ts = ThemeSet::load_defaults();
                                // let syntax = ps.find_syntax_by_name("HTML").unwrap();
                                // let mut h =
                                //     HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
                                // let regions = h.highlight_line(remainder, &ps).unwrap();
                                // raw = Some(
                                //     styled_line_to_highlighted_html(
                                //         &regions[..],
                                //         IncludeBackground::No,
                                //     )
                                //     .unwrap(),
                                // );

                                language = Some(key.as_ref().unwrap().to_string())
                            }
                        },
                    }
                }
            });
        }
        _ => {}
    }
    let attributes: Option<Vec<SectionAttribute>> = if tmp_attrs.len() > 0 {
        Some(tmp_attrs)
    } else {
        None
    };
    let attributes_string = Some(attributes_basic(&attributes));
    Ok((
        remainder,
        Section::CodeSection {
            attributes,
            attributes_string,
            language,
            title,
            // raw: Some(html_escape::encode_text(remainder).to_string()),
            raw,
        },
    ))
}

#[cfg(test)]
mod test {

    // use crate::block::block::*;
    use crate::section::code::*;
    // use crate::section::section::*;
    // use crate::section::section_attributes::*;
    // use crate::snippet::snippet_enum::*;

    // #[ignore]
    #[test]
    pub fn core_test_code() {
        let source = ["Bring your best compass", "Cap the jar"]
            .join("\n")
            .to_string();
        let expected = Section::CodeSection {
            attributes: None,
            attributes_string: Some("".to_string()),
            language: None,
            title: None,
            raw: Some(source.to_string()),
        };
        let results = code(&source).unwrap().1;
        assert_eq!(expected, results);
    }

    #[test]
    pub fn code_with_langauge() {
        let source = [">> HTML", "", "Cap the jar"].join("\n").to_string();
        let expected = Section::CodeSection {
            attributes: None,
            attributes_string: Some("".to_string()),
            language: Some("HTML".to_string()),
            title: None,
            raw: Some("Cap the jar".to_string()),
        };
        let results = code(&source).unwrap().1;
        assert_eq!(expected, results);
    }

    // #[ignore]
    #[test]
    pub fn code_with_title() {
        let source = [">> title: Some new title", "", "Cap the jar"]
            .join("\n")
            .to_string();
        let expected = Section::CodeSection {
            attributes: Some(vec![SectionAttribute::Attribute {
                key: Some("title".to_string()),
                value: Some("Some new title".to_string()),
            }]),
            attributes_string: Some(r#" title="Some new title""#.to_string()),
            language: None,
            title: Some("Some new title".to_string()),
            raw: Some("Cap the jar".to_string()),
        };
        let results = code(&source).unwrap().1;
        assert_eq!(expected, results);
    }

    // NOTE: Code highlighting has been removed because it
    // was very slow.
    // #[test]
    // pub fn code_with_highlights() {
    //     let source = [">> HTML", "", "Cap the jar"].join("\n").to_string();
    //     let expected = Section::CodeSection {
    //         attributes: None,
    //         attributes_string: Some("".to_string()),
    //         language: Some("HTML".to_string()),
    //         title: None,
    //         raw: Some("<span style=\"color:#c0c5ce;\">Cap the jar</span>".to_string()),
    //     };
    //     let results = code(&source).unwrap().1;
    //     assert_eq!(expected, results);
    // }
}
