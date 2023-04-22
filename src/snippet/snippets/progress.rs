use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;

pub fn progress(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<progress");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</progress>");
    Snippet::ProgressTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::snippets::progress::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::ProgressTag {
            string: Some("<progress>Set the piece</progress>".to_string()),
        };
        let results = progress("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::ProgressTag {
            string: Some(r#"<progress class="alfa">Pile the coal</progress>"#.to_string()),
        };
        let results = progress("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::ProgressTag {
            string: Some(
                r#"<progress id="echo" class="delta foxtrot">Raise the sail</progress>"#.to_string(),
            ),
        };
        let results = progress("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::ProgressTag {
            string: Some(r#"<progress checked>Lift the stone</progress>"#.to_string()),
        };
        let results = progress("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
