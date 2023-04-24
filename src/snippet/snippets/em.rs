use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;
use html_escape;

pub fn em(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<em");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    let escaped_text = html_escape::encode_text(text).to_string();
    response.push_str(escaped_text.as_str());
    response.push_str("</em>");
    Snippet::EmphasisTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::snippet_enum::Snippet;
    use crate::snippet::snippets::em::*;

    #[test]
    fn basic() {
        let expected = Snippet::EmphasisTag {
            string: Some("<em>Set the piece</em>".to_string()),
        };
        let results = em("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::EmphasisTag {
            string: Some(r#"<em class="alfa">Pile the coal</em>"#.to_string()),
        };
        let results = em("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::EmphasisTag {
            string: Some(r#"<em id="echo" class="delta foxtrot">Raise the sail</em>"#.to_string()),
        };
        let results = em("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::EmphasisTag {
            string: Some(r#"<em checked>Lift the stone</em>"#.to_string()),
        };
        let results = em("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
