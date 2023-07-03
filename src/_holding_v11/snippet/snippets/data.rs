use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;
use html_escape;

pub fn data(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<data");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    let escaped_text = html_escape::encode_text(text).to_string();
    response.push_str(escaped_text.as_str());
    response.push_str("</data>");
    Snippet::DataTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::snippets::data::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::DataTag {
            string: Some("<data>Set the piece</data>".to_string()),
        };
        let results = data("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::DataTag {
            string: Some(r#"<data class="alfa">Pile the coal</data>"#.to_string()),
        };
        let results = data("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::DataTag {
            string: Some(
                r#"<data id="echo" class="delta foxtrot">Raise the sail</data>"#.to_string(),
            ),
        };
        let results = data("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::DataTag {
            string: Some(r#"<data checked>Lift the stone</data>"#.to_string()),
        };
        let results = data("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
