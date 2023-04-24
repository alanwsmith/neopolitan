use crate::section::section::*;
use crate::section::section_attributes::*;
use crate::section::lib::get_title_from_attributes::*;
use nom::character::complete::multispace0;
use nom::IResult;
use html_escape;

pub fn code(source: &str) -> IResult<&str, Section> {
    let (remainder, attributes) = section_attributes(source)?;
    let (remainder, _) = multispace0(remainder)?;
    let title = get_title_from_attributes(&attributes);
    Ok((
        remainder,
        Section::CodeSection {
            attributes,
            attributes_string: None,
            language: None,
            title,
            raw: Some(html_escape::encode_text(remainder).to_string()),
        },
    ))
}

#[cfg(test)]
mod test {

    // use crate::block::block::*;
    use crate::section::code::code;
    use crate::section::section::*;
    // use crate::section::section_attributes::*;
    // use crate::snippet::snippet_enum::*;

    #[test]
    pub fn core_test_code() {
        let source = ["Bring your best compass", "Cap the jar"]
            .join("\n")
            .to_string();
        let expected = Section::CodeSection {
            attributes: None,
            attributes_string: None,
            language: None,
            title: None,
            raw: Some(source.to_string()),
        };
        let results = code(&source).unwrap().1;
        assert_eq!(expected, results);
  }

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
