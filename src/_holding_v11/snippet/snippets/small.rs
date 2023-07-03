use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;
use html_escape;

pub fn small(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<small");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    let escaped_text = html_escape::encode_text(text).to_string();
    response.push_str(escaped_text.as_str());
    response.push_str("</small>");
    Snippet::SmallTextTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::snippets::small::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::SmallTextTag {
            string: Some("<small>Set the piece</small>".to_string()),
        };
        let results = small("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::SmallTextTag {
            string: Some(r#"<small class="alfa">Pile the coal</small>"#.to_string()),
        };
        let results = small("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::SmallTextTag {
            string: Some(
                r#"<small id="echo" class="delta foxtrot">Raise the sail</small>"#.to_string(),
            ),
        };
        let results = small("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::SmallTextTag {
            string: Some(r#"<small checked>Lift the stone</small>"#.to_string()),
        };
        let results = small("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
