use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;

pub fn kbd(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<kbd");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</kbd>");
    Snippet::KeyboardInput {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::snippets::kbd::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::KeyboardInput {
            string: Some("<kbd>Set the piece</kbd>".to_string()),
        };
        let results = kbd("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::KeyboardInput {
            string: Some(r#"<kbd class="alfa">Pile the coal</kbd>"#.to_string()),
        };
        let results = kbd("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::KeyboardInput {
            string: Some(
                r#"<kbd id="echo" class="delta foxtrot">Raise the sail</kbd>"#.to_string(),
            ),
        };
        let results = kbd("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::KeyboardInput {
            string: Some(r#"<kbd checked>Lift the stone</kbd>"#.to_string()),
        };
        let results = kbd("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
