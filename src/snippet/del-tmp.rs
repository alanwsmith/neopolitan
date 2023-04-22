use crate::snippet::get_attributes::get_attributes;
use crate::snippet::snippet::Snippet;

pub fn del(text: &str, raw_attribute_string: &str) -> Snippet {
    let attributes = get_attributes(raw_attribute_string);
    let mut response = String::from("<del");
    if let Some(x) = attributes.unwrap().1 {
        response.push_str(x.as_str());
    };
    response.push_str(">");
    response.push_str(text);
    response.push_str("</del>");
    Snippet::DeleteTag {
        string: Some(response.to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::snippet::del::*;
    use crate::snippet::snippet::Snippet;

    #[test]
    fn basic() {
        let expected = Snippet::DeleteTag {
            string: Some("<del>Set the piece</del>".to_string()),
        };
        let results = del("Set the piece", "");
        assert_eq!(expected, results);
    }

    #[test]
    fn one_attribute() {
        let expected = Snippet::DeleteTag {
            string: Some(r#"<del class="alfa">Pile the coal</del>"#.to_string()),
        };
        let results = del("Pile the coal", r#"class: alfa"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn two_attribute() {
        let expected = Snippet::DeleteTag {
            string: Some(
                r#"<del id="echo" class="delta foxtrot">Raise the sail</del>"#.to_string(),
            ),
        };
        let results = del("Raise the sail", r#"id: echo|class: delta foxtrot"#);
        assert_eq!(expected, results);
    }

    #[test]
    fn just_a_key() {
        let expected = Snippet::DeleteTag {
            string: Some(r#"<del checked>Lift the stone</del>"#.to_string()),
        };
        let results = del("Lift the stone", r#"checked"#);
        assert_eq!(expected, results);
    }
}
