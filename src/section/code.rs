use crate::section::lib::get_title_from_attributes::*;
use crate::section::section::*;
use crate::section::section_attributes::*;
use crate::source_file::attributes_basic::attributes_basic;
use html_escape;
use nom::character::complete::multispace0;
use nom::IResult;
// use syntect::html;

// pub fn code_old(source: &str) -> IResult<&str, Section> {
//     let (remainder, attributes) = section_attributes(source)?;
//     let (remainder, _) = multispace0(remainder)?;
//     let title = get_title_from_attributes(&attributes);
//     // let escaped_text = html_escape::encode_text(remainder).to_string();
//     Ok((
//         remainder,
//         Section::CodeSection {
//             attributes,
//             attributes_string: None,
//             language: None,
//             title,
//             raw: Some(html_escape::encode_text(remainder).to_string()),
//         },
//     ))
// }

pub fn code(source: &str) -> IResult<&str, Section> {
    let (remainder, initial_attributes) = section_attributes(source)?;
    let (remainder, _) = multispace0(remainder)?;
    let title = get_title_from_attributes(&initial_attributes);
    let mut language: Option<String> = None;
    // This pulls off the first attribute if there is no
    // value and uses it as the language for code highlighting
    let mut tmp_attrs: Vec<SectionAttribute> = vec![];
    match initial_attributes {
        Some(attrs) => {
            attrs.iter().enumerate().for_each(|(a, b)| {
                if a == 0 {
                    match b {
                        SectionAttribute::Attribute { key, value } => match value {
                            Some(_) => tmp_attrs.push(SectionAttribute::Attribute {
                                key: Some(key.as_ref().unwrap().to_string()),
                                value: Some(value.as_ref().unwrap().to_string()),
                            }),
                            None => language = Some("HTML".to_string()),
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
            raw: Some(html_escape::encode_text(remainder).to_string()),
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

    // #[test]
    // pub fn code_with_syntax_highlights() {
    //     let source = ["<h1>Alfa</h1>"].join("\n").to_string();
    //     let expected = Section::CodeSection {
    //         attributes: None,
    //         attributes_string: None,
    //         language: None,
    //         title: None,
    //         raw: Some(source.to_string()),
    //     };
    //     let results = code(&source).unwrap().1;
    //     assert_eq!(expected, results);
    // }

    //   #[test]
    //   pub fn code_with_name() {
    //       let source = [">> name: tango", "", "Bring your best compass", "Cap the jar"]
    //           .join("\n")
    //           .to_string();
    //       let expected = Section::CodeSection {
    //           attributes: Some(vec![
    //             SectionAttribute::Attribute {
    //                 key: Some("name".to_string()),
    //                 value: Some("tango".to_string()),
    //             }
    //           ]),
    //           attributes_string: None,
    //           language: None,
    //           name: Some("tango".to_string()),
    //           raw: Some("Bring your best compass\nCap the jar".to_string())

    //         };
    //       let results = code(&source).unwrap().1;
    //       assert_eq!(expected, results);

    // }

    // #[test]
    // pub fn attributes_with_code() {
    //     let source = [
    //         "-> code",
    //         ">> class: alfa",
    //         "",
    //         "Bring your best compass",
    //         "Cap the jar",
    //     ]
    //     .join("\n")
    //     .to_string();
    //     let expected = Some(
    //         vec![
    //             r#"<pre><code class="alfa">Bring your best compass"#,
    //             r#"Cap the jar</code></pre>"#,
    //         ]
    //         .join("\n")
    //         .to_string(),
    //     );
    //     let mut u = Universe::new();
    //     u.env = Some(create_env("./site/templates"));
    //     let mut sf = SourceFile::new();
    //     sf.raw = Some(source);
    //     sf.parsed = parse(sf.raw.as_ref().unwrap().as_str()).unwrap().1;
    //     let output = sf.output(&u);
    //     assert_eq!(remove_whitespace(expected), remove_whitespace(output),);
    // }
}
