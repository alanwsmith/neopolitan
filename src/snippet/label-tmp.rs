use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet::Snippet;

pub fn label(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<label");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</label>");
    Snippet::LabelTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::label::*;
    use crate::snippet::snippet::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::LabelTag {
            string: Some("<label>Set the piece</label>".to_string()),
        };
        let results = label("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::LabelTag {
            string: Some(r#"<label class="alfa">Pile the coal</label>"#.to_string()),
        };
        let results = label("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::LabelTag {
            string: Some(
                r#"<label id="echo" class="delta foxtrot">Raise the sail</label>"#.to_string(),
            ),
        };
        let results = label("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::LabelTag {
            string: Some(r#"<label checked>Lift the stone</label>"#.to_string()),
        };
        let results = label("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
