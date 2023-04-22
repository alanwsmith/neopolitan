use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;

pub fn b(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<b");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</b>");
    Snippet::BringAttentionTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::b::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::BringAttentionTag {
            string: Some("<b>Set the piece</b>".to_string()),
        };
        let results = b("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::BringAttentionTag {
            string: Some(r#"<b class="alfa">Pile the coal</b>"#.to_string()),
        };
        let results = b("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::BringAttentionTag {
            string: Some(
                r#"<b id="echo" class="delta foxtrot">Raise the sail</b>"#.to_string(),
            ),
        };
        let results = b("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::BringAttentionTag {
            string: Some(r#"<b checked>Lift the stone</b>"#.to_string()),
        };
        let results = b("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
