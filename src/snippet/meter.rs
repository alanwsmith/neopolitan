use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;

pub fn meter(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<meter");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</meter>");
    Snippet::MeterTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::meter::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::MeterTag {
            string: Some("<meter>Set the piece</meter>".to_string()),
        };
        let results = meter("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::MeterTag {
            string: Some(r#"<meter class="alfa">Pile the coal</meter>"#.to_string()),
        };
        let results = meter("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::MeterTag {
            string: Some(
                r#"<meter id="echo" class="delta foxtrot">Raise the sail</meter>"#.to_string(),
            ),
        };
        let results = meter("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::MeterTag {
            string: Some(r#"<meter checked>Lift the stone</meter>"#.to_string()),
        };
        let results = meter("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
