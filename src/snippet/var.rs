use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet_enum::Snippet;

pub fn var(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<var");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</var>");
    Snippet::VariableTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::var::*;
    use crate::snippet::snippet_enum::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::VariableTag {
            string: Some("<var>Set the piece</var>".to_string()),
        };
        let results = var("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::VariableTag {
            string: Some(r#"<var class="alfa">Pile the coal</var>"#.to_string()),
        };
        let results = var("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::VariableTag {
            string: Some(
                r#"<var id="echo" class="delta foxtrot">Raise the sail</var>"#.to_string(),
            ),
        };
        let results = var("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::VariableTag {
            string: Some(r#"<var checked>Lift the stone</var>"#.to_string()),
        };
        let results = var("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
