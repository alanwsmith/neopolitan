use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;

pub fn legend(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<legend");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</legend>");
    Snippet::LegendTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::legend::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::LegendTag {
            string: Some("<legend>Set the piece</legend>".to_string()),
        };
        let results = legend("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::LegendTag {
            string: Some(r#"<legend class="alfa">Pile the coal</legend>"#.to_string()),
        };
        let results = legend("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::LegendTag {
            string: Some(
                r#"<legend id="echo" class="delta foxtrot">Raise the sail</legend>"#.to_string(),
            ),
        };
        let results = legend("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::LegendTag {
            string: Some(r#"<legend checked>Lift the stone</legend>"#.to_string()),
        };
        let results = legend("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
