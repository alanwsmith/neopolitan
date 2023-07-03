use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;
use html_escape;

pub fn abbr(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<abbr");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    let escaped_text = html_escape::encode_text(text).to_string();
    response.push_str(escaped_text.as_str());
    response.push_str("</abbr>");
    Snippet::AbbreviationTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::snippet_enum::Snippet;
    use crate::snippet::snippets::abbr::*;

    #[test]
    fn basic() {
        let expected = Snippet::AbbreviationTag {
            string: Some("<abbr>Set the piece</abbr>".to_string()),
        };
        let results = abbr("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::AbbreviationTag {
            string: Some(r#"<abbr class="alfa">Pile the coal</abbr>"#.to_string()),
        };
        let results = abbr("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::AbbreviationTag {
            string: Some(
                r#"<abbr id="echo" class="delta foxtrot">Raise the sail</abbr>"#.to_string(),
            ),
        };
        let results = abbr("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::AbbreviationTag {
            string: Some(r#"<abbr checked>Lift the stone</abbr>"#.to_string()),
        };
        let results = abbr("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
