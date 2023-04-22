use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;

pub fn button(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<button");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</button>");
    Snippet::ButtonTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::button::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::ButtonTag {
            string: Some("<button>Set the piece</button>".to_string()),
        };
        let results = button("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::ButtonTag {
            string: Some(r#"<button class="alfa">Pile the coal</button>"#.to_string()),
        };
        let results = button("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::ButtonTag {
            string: Some(
                r#"<button id="echo" class="delta foxtrot">Raise the sail</button>"#.to_string(),
            ),
        };
        let results = button("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::ButtonTag {
            string: Some(r#"<button checked>Lift the stone</button>"#.to_string()),
        };
        let results = button("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
