use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;

pub fn u(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<u");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</u>");
    Snippet::UnarticulatedAnnotationTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::u::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::UnarticulatedAnnotationTag {
            string: Some("<u>Set the piece</u>".to_string()),
        };
        let results = u("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::UnarticulatedAnnotationTag {
            string: Some(r#"<u class="alfa">Pile the coal</u>"#.to_string()),
        };
        let results = u("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::UnarticulatedAnnotationTag {
            string: Some(
                r#"<u id="echo" class="delta foxtrot">Raise the sail</u>"#.to_string(),
            ),
        };
        let results = u("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::UnarticulatedAnnotationTag {
            string: Some(r#"<u checked>Lift the stone</u>"#.to_string()),
        };
        let results = u("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
