use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;
use html_escape;

pub fn time(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<time");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    let escaped_text = html_escape::encode_text(text).to_string();
    response.push_str(escaped_text.as_str());
    response.push_str("</time>");
    Snippet::TimeTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::snippets::time::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::TimeTag {
            string: Some("<time>Set the piece</time>".to_string()),
        };
        let results = time("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::TimeTag {
            string: Some(r#"<time class="alfa">Pile the coal</time>"#.to_string()),
        };
        let results = time("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::TimeTag {
            string: Some(
                r#"<time id="echo" class="delta foxtrot">Raise the sail</time>"#.to_string(),
            ),
        };
        let results = time("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::TimeTag {
            string: Some(r#"<time checked>Lift the stone</time>"#.to_string()),
        };
        let results = time("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
