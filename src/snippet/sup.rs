use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;

pub fn sup(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<sup");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</sup>");
    Snippet::SuperscriptTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::sup::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::SuperscriptTag {
            string: Some("<sup>Set the piece</sup>".to_string()),
        };
        let results = sup("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::SuperscriptTag {
            string: Some(r#"<sup class="alfa">Pile the coal</sup>"#.to_string()),
        };
        let results = sup("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::SuperscriptTag {
            string: Some(
                r#"<sup id="echo" class="delta foxtrot">Raise the sail</sup>"#.to_string(),
            ),
        };
        let results = sup("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::SuperscriptTag {
            string: Some(r#"<sup checked>Lift the stone</sup>"#.to_string()),
        };
        let results = sup("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
