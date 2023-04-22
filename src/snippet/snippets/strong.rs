use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;

pub fn strong(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<strong");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</strong>");
    Snippet::StrongTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::snippets::strong::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::StrongTag {
            string: Some("<strong>Set the piece</strong>".to_string()),
        };
        let results = strong("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::StrongTag {
            string: Some(r#"<strong class="alfa">Pile the coal</strong>"#.to_string()),
        };
        let results = strong("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::StrongTag {
            string: Some(
                r#"<strong id="echo" class="delta foxtrot">Raise the sail</strong>"#.to_string(),
            ),
        };
        let results = strong("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::StrongTag {
            string: Some(r#"<strong checked>Lift the stone</strong>"#.to_string()),
        };
        let results = strong("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
