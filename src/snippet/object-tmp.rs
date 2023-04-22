use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet::Snippet;

pub fn object(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<object");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</object>");
    Snippet::ObjectTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::object::*;
    use crate::snippet::snippet::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::ObjectTag {
            string: Some("<object>Set the piece</object>".to_string()),
        };
        let results = object("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::ObjectTag {
            string: Some(r#"<object class="alfa">Pile the coal</object>"#.to_string()),
        };
        let results = object("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::ObjectTag {
            string: Some(
                r#"<object id="echo" class="delta foxtrot">Raise the sail</object>"#.to_string(),
            ),
        };
        let results = object("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::ObjectTag {
            string: Some(r#"<object checked>Lift the stone</object>"#.to_string()),
        };
        let results = object("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
