use std::vec;

use crate::source_file::sections::p_blocks::p_blocks;
use nom::IResult;

pub fn aside_section<'a>(
    source: &'a str,
) -> IResult<&str, Vec<String>> {
    let mut content = p_blocks(source).unwrap().1;
    let mut output = vec!["<aside>".to_string()];
    output.append(&mut content);
    output.push("</aside>".to_string());
    Ok(("", output))
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    pub fn solo_test_basic_aside() {
        let lines = vec![
            "",
            "Slide the tray",
            "across the glass top",
            "",
            "Add salt before you fry the egg",
        ];
        assert_eq!(
            aside_section(lines.join("\n").as_str())
                .unwrap()
                .1,
            vec![
                r#"<aside>"#.to_string(),
                r#"<p>Slide the tray across the glass top</p>"#.to_string(),
                r#"<p>Add salt before you fry the egg</p>"#.to_string(),
                r#"</aside>"#.to_string(),

            ]
        );
    }
}
