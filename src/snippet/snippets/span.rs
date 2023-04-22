use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;

pub fn span(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<span");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</span>");
    Snippet::SpanTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::snippets::span::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::SpanTag {
            string: Some("<span>Set the piece</span>".to_string()),
        };
        let results = span("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::SpanTag {
            string: Some(r#"<span class="alfa">Pile the coal</span>"#.to_string()),
        };
        let results = span("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::SpanTag {
            string: Some(
                r#"<span id="echo" class="delta foxtrot">Raise the sail</span>"#.to_string(),
            ),
        };
        let results = span("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::SpanTag {
            string: Some(r#"<span checked>Lift the stone</span>"#.to_string()),
        };
        let results = span("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
