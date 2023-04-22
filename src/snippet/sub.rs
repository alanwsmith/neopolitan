use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;

pub fn sub(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<sub");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</sub>");
    Snippet::SubscriptTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::sub::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::SubscriptTag {
            string: Some("<sub>Set the piece</sub>".to_string()),
        };
        let results = sub("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::SubscriptTag {
            string: Some(r#"<sub class="alfa">Pile the coal</sub>"#.to_string()),
        };
        let results = sub("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::SubscriptTag {
            string: Some(
                r#"<sub id="echo" class="delta foxtrot">Raise the sail</sub>"#.to_string(),
            ),
        };
        let results = sub("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::SubscriptTag {
            string: Some(r#"<sub checked>Lift the stone</sub>"#.to_string()),
        };
        let results = sub("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
